use crate::common::characters::CharacterOf;
use crate::common::error::JikanError;
use crate::common::request::RequestMetadata;
use crate::common::voice_actor::VoiceActor;
use crate::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Info {
  about: String,
  animeography: Vec<CharacterOf>,
  #[serde(alias = "mal_id")]
  id: u32,
  image_url: Url,
  mangaography: Vec<CharacterOf>,
  member_favorites: u32,
  #[serde(flatten)]
  metadata: RequestMetadata,
  name: String,
  name_kanji: String,
  nicknames: Vec<String>,
  url: Url,
  voice_actors: Vec<VoiceActor>,
}

impl Info {
  pub fn get_url_path(id: u32) -> String {
    format!("/character/{}", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Info::get_url_path(id)).await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::test_helper::{self, CharacterTestSuite};
  use super::*;
  use crate::utils::test_helper as utils_test_helper;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_info_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for CharacterTestSuite { id, name } in test_helper::get_valid_characters() {
      let mock = server.mock(|when, then| {
        when.path(Info::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/character/__test__/info_{}.json",
            id
          )));
      });

      let info = Info::from_id(&client, id).await;
      mock.assert();
      assert_eq!(info?.id(), &id, "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_info_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for CharacterTestSuite { id, name } in test_helper::get_invalid_characters() {
      let mock = server.mock(|when, then| {
        when.path(Info::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/character/__test__/info_{}.json",
            id
          )));
      });

      let info = Info::from_id(&client, id).await;
      mock.assert();
      assert!(
        info.is_err(),
        "Response for character \"{}\" is not 404",
        name
      );
    }

    Ok(())
  }
}

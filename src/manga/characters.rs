use crate::common::error::JikanError;
use crate::common::request::RequestMetadata;
use crate::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum CharacterRole {
  Main,
  Supporting,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Character {
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: String,
  name: String,
  role: CharacterRole,
  url: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Characters {
  #[serde(flatten)]
  metadata: RequestMetadata,
  characters: Vec<Character>,
}

impl Characters {
  pub fn get_url_path(id: u32) -> String {
    format!("/manga/{}/characters", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Characters::get_url_path(id)).await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::test_helper::{self, MangaTestSuite};
  use super::*;
  use crate::utils::test_helper as utils_test_helper;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_characters_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_valid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Characters::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/characters_{}.json",
            id
          )));
      });

      let characters = Characters::from_id(&client, id).await;
      mock.assert();
      assert!(characters.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_characters_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_invalid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Characters::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/characters_{}.json",
            id
          )));
      });

      let info = Characters::from_id(&client, id).await;
      mock.assert();
      assert!(info.is_err(), "Response for manga \"{}\" is not 404", name);
    }

    Ok(())
  }
}

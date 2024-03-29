use crate::v3::common::characters::CharacterRole;
use crate::v3::common::error::JikanError;
use crate::v3::common::request::RequestMetadata;
use crate::v3::common::staff::StaffPosition;
use crate::v3::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct EntityBriefInfo {
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: Url,
  name: String,
  url: Url,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct AnimeStaffPosition {
  anime: EntityBriefInfo,
  position: StaffPosition,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum MangaPosition {
  Art,
  Story,
  #[serde(alias = "Story & Art")]
  StoryAndArt,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct PublishedManga {
  manga: EntityBriefInfo,
  position: MangaPosition,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct VoiceActingPosition {
  anime: EntityBriefInfo,
  character: EntityBriefInfo,
  role: CharacterRole,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Info {
  about: String,
  alternate_names: Vec<String>,
  anime_staff_positions: Vec<AnimeStaffPosition>,
  birthday: DateTime<FixedOffset>,
  family_name: String,
  given_name: String,
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: Url,
  member_favorites: u32,
  #[serde(flatten)]
  metadata: RequestMetadata,
  name: String,
  published_manga: Vec<PublishedManga>,
  url: Url,
  voice_acting_roles: Vec<VoiceActingPosition>,
  website_url: Option<String>,
}

impl Info {
  pub fn get_url_path(id: u32) -> String {
    format!("/person/{}", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Info::get_url_path(id)).await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::test_helper::{self, PersonTestSuite};
  use super::*;
  use crate::v3::utils::test_helper as utils_test_helper;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_info_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for PersonTestSuite { id, name } in test_helper::get_valid_persons() {
      let mock = server.mock(|when, then| {
        when.path(Info::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/v3/person/__test__/info_{}.json",
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

    for PersonTestSuite { id, name } in test_helper::get_invalid_persons() {
      let mock = server.mock(|when, then| {
        when.path(Info::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/v3/person/__test__/info_{}.json",
            id
          )));
      });

      let info = Info::from_id(&client, id).await;
      mock.assert();
      assert!(info.is_err(), "Response for person \"{}\" is not 404", name);
    }

    Ok(())
  }
}

use super::super::common::error::JikanError;
use super::super::common::tag::Tag;
use super::super::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct PublishingDetail {
  from: Option<DateTime<FixedOffset>>,
  #[serde(rename = "string")]
  text: String,
  to: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum PublishingStatus {
  Finished,
  Publishing,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct MangaRelated {
  #[serde(rename = "Alternative version")]
  alternative_version: Option<Vec<Tag>>,
  #[serde(rename = "Adaptation")]
  adaptation: Option<Vec<Tag>>,
  #[serde(rename = "Other")]
  other: Option<Vec<Tag>>,
  #[serde(rename = "Side story")]
  side_story: Option<Vec<Tag>>,
  #[serde(rename = "Spin-off")]
  spin_off: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum MangaType {
  Doujinshi,
  #[serde(alias = "Light Novel")]
  LightNovel,
  Manga,
  Manhwa,
  Manhua,
  #[serde(alias = "One-shot")]
  OneShot,
  #[serde(other)]
  Other,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Info {
  authors: Vec<Tag>,
  background: Option<String>,
  chapters: Option<u32>,
  favorites: u32,
  genres: Vec<Tag>,
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: String,
  members: u32,
  popularity: u32,
  published: PublishingDetail,
  publishing: bool,
  r#type: MangaType,
  rank: Option<u32>,
  related: MangaRelated,
  score: Option<f32>,
  scored_by: Option<u32>,
  serializations: Vec<Tag>,
  status: PublishingStatus,
  synopsis: String,
  title_english: Option<String>,
  title_japanese: String,
  title_synonyms: Vec<String>,
  title: String,
  url: String,
  volumes: Option<u32>,
}

impl Info {
  pub fn get_url_path(id: u32) -> String {
    format!("/manga/{}", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Info::get_url_path(id)).await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::super::utils::test_helper as utils_test_helper;
  use super::super::test_helper::{self, MangaTestSuite};
  use super::*;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_info_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_valid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Info::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/info_{}.json",
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

    for MangaTestSuite { id, name } in test_helper::get_invalid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Info::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/info_{}.json",
            id
          )));
      });

      let info = Info::from_id(&client, id).await;
      mock.assert();
      assert!(info.is_err(), "Response for manga \"{}\" is not 404", name);
    }

    Ok(())
  }
}

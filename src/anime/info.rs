use crate::common::error::JikanError;
use crate::common::request::RequestMetadata;
use crate::common::tag::Tag;
use crate::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct AiringDetail {
  from: Option<DateTime<FixedOffset>>,
  #[serde(rename = "string")]
  text: String,
  to: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct AnimeRelated {
  #[serde(rename = "Alternative version")]
  alternative_version: Option<Vec<Tag>>,
  #[serde(rename = "Adaptation")]
  adaptation: Option<Vec<Tag>>,
  #[serde(rename = "Side story")]
  side_story: Option<Vec<Tag>>,
  #[serde(rename = "Sequel")]
  sequel: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum AiringStatus {
  #[serde(alias = "Currently Airing")]
  CurrentlyAiring,
  #[serde(alias = "Finished Airing")]
  FinishedAiring,
  #[serde(alias = "Not yet aired")]
  NotYetAired,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum AnimeSource {
  #[serde(alias = "Light novel")]
  LightNovel,
  Manga,
  Music,
  Original,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum AnimeType {
  #[serde(alias = "OVA")]
  Ova,
  #[serde(alias = "TV")]
  Tv,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Info {
  aired: AiringDetail,
  #[serde(rename = "status")]
  airing_status: AiringStatus,
  background: Option<String>,
  broadcast: Option<String>,
  duration: String,
  ending_themes: Vec<String>,
  episodes: Option<u32>,
  favorites: u32,
  genres: Vec<Tag>,
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: String,
  #[serde(rename = "airing")]
  is_airing: bool,
  licensors: Vec<Tag>,
  members: u32,
  #[serde(flatten)]
  metadata: RequestMetadata,
  opening_themes: Vec<String>,
  popularity: u32,
  premiered: Option<String>,
  producers: Vec<Tag>,
  rank: Option<u32>,
  rating: String,
  related: AnimeRelated,
  score: Option<f32>,
  scored_by: Option<u32>,
  source: AnimeSource,
  studios: Vec<Tag>,
  synopsis: String,
  title_english: Option<String>,
  title_japanese: String,
  title_synonyms: Vec<String>,
  title: String,
  trailer_url: Option<String>,
  r#type: AnimeType,
  url: String,
}

impl Info {
  pub fn get_url_path(id: u32) -> String {
    format!("/anime/{}", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Info::get_url_path(id)).await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::test_helper::{self, AnimeTestSuite};
  use super::*;
  use crate::utils::test_helper as utils_test_helper;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_info_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(Info::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/info_{}.json",
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

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mock = server.mock(|when, then| {
        when.path(Info::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/info_{}.json",
            id
          )));
      });

      let info = Info::from_id(&client, id).await;
      mock.assert();
      assert!(info.is_err(), "Response for anime \"{}\" is not 404", name);
    }

    Ok(())
  }
}

use super::super::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct AiringDetail {
  from: Option<DateTime<FixedOffset>>,
  #[serde(rename = "string")]
  text: String,
  to: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum TagType {
  #[serde(alias = "anime")]
  Anime,
  #[serde(alias = "manga")]
  Manga,
  #[serde(other)]
  Other,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Tag {
  #[serde(rename = "mal_id")]
  id: u32,
  r#type: TagType,
  name: String,
  url: String,
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
  Other,
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
  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, Box<dyn Error>> {
    let response = client.get::<Self>(&format!("/anime/{}", id)).await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::{test_helper, test_helper::AnimeTestSuite};
  use super::*;
  use serial_test::serial;
  use std::error::Error;
  use std::thread;

  #[tokio::test]
  #[serial]
  async fn can_get_info_by_id() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let info = Info::from_id(&client, id).await;

      match info {
        Ok(_) => assert_eq!(info?.id(), &id, "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_handle_anime_404() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      assert!(
        Info::from_id(&client, id).await.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }
}

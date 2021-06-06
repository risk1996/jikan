use crate::common::error::JikanError;
use crate::common::request::RequestMetadata;
use crate::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Episode {
  #[serde(rename = "aired")]
  aired_at: Option<DateTime<FixedOffset>>,
  #[serde(rename = "episode_id")]
  id: u32,
  filler: bool,
  forum_url: Option<String>,
  recap: bool,
  title: String,
  title_japanese: Option<String>,
  title_romanji: Option<String>,
  video_url: Option<String>,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Episodes {
  episodes: Vec<Episode>,
  episodes_last_page: u32,
  #[serde(flatten)]
  metadata: RequestMetadata,
}

impl Episodes {
  pub fn get_url_path(id: u32, page: u32) -> String {
    format!("/anime/{}/episodes/{}", id, page)
  }

  pub async fn from_id_at_page(
    client: &JikanHttpClient,
    id: u32,
    page: u32,
  ) -> Result<Self, JikanError> {
    let response = client
      .get::<Self>(&Episodes::get_url_path(id, page))
      .await?;
    Ok(response.into_body())
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    Episodes::from_id_at_page(client, id, 1).await
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
  async fn can_get_episodes_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());
    let page = 1;

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(Episodes::get_url_path(id, page));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/episodes_{}_page_{}.json",
            id, page
          )));
      });

      let episodes = Episodes::from_id(&client, id).await;
      mock.assert();
      assert!(episodes.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_episodes_empty_episodes() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());
    let page = 1;

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mock = server.mock(|when, then| {
        when.path(Episodes::get_url_path(id, page));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/episodes_{}_page_{}.json",
            id, page
          )));
      });

      let episodes = Episodes::from_id(&client, id).await;
      mock.assert();
      assert_eq!(
        episodes?.episodes.len(),
        0,
        "Episodes for anime \"{}\" is not 0",
        name,
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_get_episodes_by_id_at_page() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      for page in test_helper::get_pages() {
        let mock = server.mock(|when, then| {
          when.path(Episodes::get_url_path(id, page));
          then
            .status(200)
            .body(utils_test_helper::file_to_string(&format!(
              "src/anime/__test__/episodes_{}_page_{}.json",
              id, page
            )));
        });

        let episodes = Episodes::from_id_at_page(&client, id, page).await;
        mock.assert();
        assert!(episodes.is_ok(), "{}", name);
      }
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_empty_episodes_at_page() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      for page in test_helper::get_pages() {
        let mock = server.mock(|when, then| {
          when.path(Episodes::get_url_path(id, page));
          then
            .status(200)
            .body(utils_test_helper::file_to_string(&format!(
              "src/anime/__test__/episodes_{}_page_{}.json",
              id, page
            )));
        });

        let episodes = Episodes::from_id_at_page(&client, id, page).await;
        mock.assert();
        assert_eq!(
          episodes?.episodes.len(),
          0,
          "Episodes for anime \"{}\" is not 0",
          name,
        );
      }
    }

    Ok(())
  }
}

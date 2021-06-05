use super::super::common::error::JikanError;
use super::super::common::stats::Scores;
use super::super::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct ListStats {
  completed: u32,
  dropped: u32,
  on_hold: u32,
  plan_to_read: u32,
  reading: u32,
  total: u32,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Stats {
  #[serde(flatten)]
  list_stats: ListStats,
  scores: Scores,
}

impl Stats {
  pub fn get_url_path(id: u32) -> String {
    format!("/manga/{}/stats", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Stats::get_url_path(id)).await?;
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
  async fn can_get_stats_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_valid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Stats::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/stats_{}.json",
            id
          )));
      });

      let stats = Stats::from_id(&client, id).await;
      mock.assert();
      assert!(stats.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_stats_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_invalid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Stats::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/stats_{}.json",
            id
          )));
      });

      let stats = Stats::from_id(&client, id).await;
      mock.assert();
      assert!(stats.is_err(), "Response for manga \"{}\" is not 404", name,);
    }

    Ok(())
  }
}

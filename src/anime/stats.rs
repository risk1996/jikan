use super::super::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Score {
  votes: u32,
  percentage: f32,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Scores {
  #[serde(alias = "1")]
  score_1: Score,
  #[serde(alias = "2")]
  score_2: Score,
  #[serde(alias = "3")]
  score_3: Score,
  #[serde(alias = "4")]
  score_4: Score,
  #[serde(alias = "5")]
  score_5: Score,
  #[serde(alias = "6")]
  score_6: Score,
  #[serde(alias = "7")]
  score_7: Score,
  #[serde(alias = "8")]
  score_8: Score,
  #[serde(alias = "9")]
  score_9: Score,
  #[serde(alias = "10")]
  score_10: Score,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct ListStats {
  completed: u32,
  dropped: u32,
  on_hold: u32,
  plan_to_watch: u32,
  total: u32,
  watching: u32,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Stats {
  #[serde(flatten)]
  list_stats: ListStats,
  scores: Scores,
}

impl Stats {
  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, Box<dyn Error>> {
    let response = client.get::<Self>(&format!("/anime/{}/stats", id)).await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::super::utils::test_helper as utils_test_helper;
  use super::super::{test_helper, test_helper::AnimeTestSuite};
  use super::*;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_stats_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(format!("/anime/{}/stats", id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/stats_{}.json",
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

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mock = server.mock(|when, then| {
        when.path(format!("/anime/{}/stats", id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/stats_{}.json",
            id
          )));
      });

      let stats = Stats::from_id(&client, id).await;
      mock.assert();
      assert!(stats.is_err(), "Response for anime \"{}\" is not 404", name,);
    }

    Ok(())
  }
}

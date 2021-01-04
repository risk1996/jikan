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
  use super::super::{test_helper, test_helper::AnimeTestSuite};
  use super::*;
  use serial_test::serial;
  use std::error::Error;
  use std::thread;

  #[tokio::test]
  #[serial]
  async fn can_get_stats_by_id() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let stats = Stats::from_id(&client, id).await;

      match stats {
        Ok(_) => assert!(stats.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_handle_stats_404() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      assert!(
        Stats::from_id(&client, id).await.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }
}

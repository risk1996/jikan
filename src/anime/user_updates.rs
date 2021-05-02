use super::super::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Status {
  Completed,
  Dropped,
  #[serde(alias = "On-Hold")]
  OnHold,
  #[serde(alias = "Plan to Watch")]
  PlanToWatch,
  Watching,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct User {
  date: DateTime<FixedOffset>,
  episodes_seen: Option<u32>,
  episodes_total: Option<u32>,
  image_url: String,
  score: Option<u8>,
  status: Status,
  url: String,
  username: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct UserUpdates {
  users: Vec<User>,
}

impl UserUpdates {
  pub async fn from_id_at_page(
    client: &JikanHttpClient,
    id: u32,
    page: u32,
  ) -> Result<Self, Box<dyn Error>> {
    let response = client
      .get::<Self>(&format!("/anime/{}/userupdates/{}", id, page))
      .await?;
    Ok(response.into_body())
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, Box<dyn Error>> {
    let response = client
      .get::<Self>(&format!("/anime/{}/userupdates", id))
      .await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::{test_helper, test_helper::AnimeTestSuite};
  use super::*;
  use rand::Rng;
  use serial_test::serial;
  use std::error::Error;
  use std::thread;

  #[tokio::test]
  #[serial]
  async fn can_get_user_updates_by_id() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let user_updates = UserUpdates::from_id(&client, id).await;

      match user_updates {
        Ok(_) => assert!(user_updates.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_handle_user_updates_404() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      assert!(
        UserUpdates::from_id(&client, id).await.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_get_user_updates_by_id_at_page() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let mut rng = rand::thread_rng();
      let page = rng.gen_range(2..10);
      let user_updates = UserUpdates::from_id_at_page(&client, id, page).await;

      match user_updates {
        Ok(_) => assert!(user_updates.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_handle_user_updates_404_at_page() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mut rng = rand::thread_rng();
      let page = rng.gen_range(2..10);
      assert!(
        UserUpdates::from_id_at_page(&client, id, page)
          .await
          .is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }
}

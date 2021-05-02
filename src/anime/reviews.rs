use super::super::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Scores {
  animation: u8,
  character: u8,
  enjoyment: u8,
  overall: u8,
  sound: u8,
  story: u8,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Reviewer {
  episodes_seen: u32,
  image_url: String,
  scores: Scores,
  url: String,
  username: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Review {
  content: String,
  date: DateTime<FixedOffset>,
  helpful_count: u32,
  #[serde(rename = "mal_id")]
  id: u32,
  reviewer: Reviewer,
  url: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Reviews {
  reviews: Vec<Review>,
}

impl Reviews {
  pub async fn from_id_at_page(
    client: &JikanHttpClient,
    id: u32,
    page: u32,
  ) -> Result<Self, Box<dyn Error>> {
    let response = client
      .get::<Self>(&format!("/anime/{}/reviews/{}", id, page))
      .await?;
    Ok(response.into_body())
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, Box<dyn Error>> {
    let response = client
      .get::<Self>(&format!("/anime/{}/reviews", id))
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
  async fn can_get_reviews_by_id() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let reviews = Reviews::from_id(&client, id).await;

      match reviews {
        Ok(_) => assert!(reviews.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_handle_reviews_404() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      assert!(
        Reviews::from_id(&client, id).await.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_get_reviews_by_id_at_page() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let mut rng = rand::thread_rng();
      let page = rng.gen_range(2..10);
      let reviews = Reviews::from_id_at_page(&client, id, page).await;

      match reviews {
        Ok(_) => assert!(reviews.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_handle_reviews_404_at_page() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mut rng = rand::thread_rng();
      let page = rng.gen_range(2..10);
      assert!(
        Reviews::from_id_at_page(&client, id, page).await.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }
}

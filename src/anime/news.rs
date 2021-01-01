use super::super::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Article {
  author_name: String,
  author_url: String,
  comments: u32,
  #[serde(rename = "date")]
  created_at: DateTime<FixedOffset>,
  forum_url: String,
  image_url: String,
  intro: String,
  title: String,
  url: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct News {
  articles: Vec<Article>,
}

impl News {
  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, Box<dyn Error>> {
    let response = client.get::<Self>(&format!("/anime/{}/news", id)).await?;
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
  async fn can_get_news_by_id() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let news = News::from_id(&client, id).await;

      match news {
        Ok(_) => assert!(news.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_handle_news_404() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      assert!(
        News::from_id(&client, id).await.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }
}

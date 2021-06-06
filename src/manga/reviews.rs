use crate::common::error::JikanError;
use crate::common::request::RequestMetadata;
use crate::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Scores {
  art: u8,
  character: u8,
  enjoyment: u8,
  overall: u8,
  story: u8,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Reviewer {
  chapters_read: u32,
  image_url: Url,
  scores: Scores,
  url: Url,
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
  url: Url,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Reviews {
  #[serde(flatten)]
  metadata: RequestMetadata,
  reviews: Vec<Review>,
}

impl Reviews {
  pub fn get_url_path(id: u32, page: u32) -> String {
    format!("/manga/{}/reviews/{}", id, page)
  }

  pub async fn from_id_at_page(
    client: &JikanHttpClient,
    id: u32,
    page: u32,
  ) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Reviews::get_url_path(id, page)).await?;
    Ok(response.into_body())
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    Reviews::from_id_at_page(client, id, 1).await
  }
}

#[cfg(test)]
mod tests {
  use super::super::test_helper::{self, MangaTestSuite};
  use super::*;
  use crate::utils::test_helper as utils_test_helper;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_reviews_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());
    let page = 1;

    for MangaTestSuite { id, name } in test_helper::get_valid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Reviews::get_url_path(id, page));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/reviews_{}_page_{}.json",
            id, page
          )));
      });

      let reviews = Reviews::from_id(&client, id).await;
      mock.assert();
      assert!(reviews.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_reviews_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());
    let page = 1;

    for MangaTestSuite { id, name } in test_helper::get_invalid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Reviews::get_url_path(id, page));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/reviews_{}_page_{}.json",
            id, page
          )));
      });

      let reviews = Reviews::from_id(&client, id).await;
      mock.assert();
      assert!(
        reviews.is_err(),
        "Response for manga \"{}\" is not 404",
        name,
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_get_reviews_by_id_at_page() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_valid_mangas() {
      for page in test_helper::get_pages() {
        let mock = server.mock(|when, then| {
          when.path(Reviews::get_url_path(id, page));
          then
            .status(200)
            .body(utils_test_helper::file_to_string(&format!(
              "src/manga/__test__/reviews_{}_page_{}.json",
              id, page
            )));
        });

        let reviews = Reviews::from_id_at_page(&client, id, page).await;
        mock.assert();
        assert!(reviews.is_ok(), "{}", name);
      }
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_reviews_404_at_page() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_invalid_mangas() {
      for page in test_helper::get_pages() {
        let mock = server.mock(|when, then| {
          when.path(Reviews::get_url_path(id, page));
          then
            .status(404)
            .body(utils_test_helper::file_to_string(&format!(
              "src/manga/__test__/reviews_{}_page_{}.json",
              id, page
            )));
        });

        let reviews = Reviews::from_id_at_page(&client, id, page).await;
        mock.assert();
        assert!(
          reviews.is_err(),
          "Response for manga \"{}\" is not 404",
          name,
        );
      }
    }

    Ok(())
  }
}

use crate::common::error::JikanError;
use crate::common::request::RequestMetadata;
use crate::common::user_updates::User;
use crate::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

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
pub struct UserUpdates {
  #[serde(flatten)]
  metadata: RequestMetadata,
  users: Vec<User<Status>>,
}

impl UserUpdates {
  pub fn get_url_path(id: u32, page: u32) -> String {
    format!("/anime/{}/userupdates/{}", id, page)
  }

  pub async fn from_id_at_page(
    client: &JikanHttpClient,
    id: u32,
    page: u32,
  ) -> Result<Self, JikanError> {
    let response = client
      .get::<Self>(&UserUpdates::get_url_path(id, page))
      .await?;
    Ok(response.into_body())
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    UserUpdates::from_id_at_page(client, id, 1).await
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
  async fn can_get_user_updates_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());
    let page = 1;

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(UserUpdates::get_url_path(id, page));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/user_updates_{}_page_{}.json",
            id, page
          )));
      });

      let user_updates = UserUpdates::from_id(&client, id).await;
      mock.assert();
      assert!(user_updates.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_user_updates_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());
    let page = 1;

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mock = server.mock(|when, then| {
        when.path(UserUpdates::get_url_path(id, page));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/user_updates_{}_page_{}.json",
            id, page
          )));
      });

      let user_updates = UserUpdates::from_id(&client, id).await;
      mock.assert();
      assert!(
        user_updates.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_get_user_updates_by_id_at_page() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      for page in test_helper::get_pages() {
        let mock = server.mock(|when, then| {
          when.path(UserUpdates::get_url_path(id, page));
          then
            .status(if page == 1 { 200 } else { 404 })
            .body(utils_test_helper::file_to_string(&format!(
              "src/anime/__test__/user_updates_{}_page_{}.json",
              id, page
            )));
        });

        let user_updates = UserUpdates::from_id_at_page(&client, id, page).await;
        mock.assert();
        if page == 1 {
          assert!(user_updates.is_ok(), "{}", name);
        } else {
          assert!(
            user_updates.is_err(),
            "Response for anime \"{}\" is not 404",
            name,
          );
        }
      }
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_user_updates_404_at_page() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      for page in test_helper::get_pages() {
        let mock = server.mock(|when, then| {
          when.path(UserUpdates::get_url_path(id, page));
          then
            .status(404)
            .body(utils_test_helper::file_to_string(&format!(
              "src/anime/__test__/user_updates_{}_page_{}.json",
              id, page
            )));
        });
        let user_updates = UserUpdates::from_id_at_page(&client, id, page).await;
        mock.assert();
        assert!(
          user_updates.is_err(),
          "Response for anime \"{}\" is not 404",
          name,
        );
      }
    }

    Ok(())
  }
}

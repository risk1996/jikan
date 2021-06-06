use crate::common::error::JikanError;
use crate::common::forum::Topic;
use crate::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Forum {
  topics: Vec<Topic>,
}

impl Forum {
  pub fn get_url_path(id: u32) -> String {
    format!("/anime/{}/forum", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Forum::get_url_path(id)).await?;
    Ok(response.into_body())
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
  async fn can_get_forum_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(Forum::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/forum_{}.json",
            id
          )));
      });

      let forum = Forum::from_id(&client, id).await;
      mock.assert();
      assert!(forum.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_forum_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mock = server.mock(|when, then| {
        when.path(Forum::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/forum_{}.json",
            id
          )));
      });

      let forum = Forum::from_id(&client, id).await;
      mock.assert();
      assert!(forum.is_err(), "Response for anime \"{}\" is not 404", name,);
    }

    Ok(())
  }
}

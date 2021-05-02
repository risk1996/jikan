use super::super::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Post {
  author_name: String,
  author_url: String,
  #[serde(rename = "date_posted")]
  created_at: DateTime<FixedOffset>,
  url: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Topic {
  author_name: String,
  author_url: String,
  #[serde(rename = "date_posted")]
  created_at: DateTime<FixedOffset>,
  #[serde(rename = "topic_id")]
  id: u32,
  last_post: Post,
  replies: u32,
  title: String,
  url: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Forum {
  topics: Vec<Topic>,
}

impl Forum {
  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, Box<dyn Error>> {
    let response = client.get::<Self>(&format!("/anime/{}/forum", id)).await?;
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
  async fn can_get_forum_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(format!("/anime/{}/forum", id));
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
        when.path(format!("/anime/{}/forum", id));
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

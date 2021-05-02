use super::super::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::error::Error;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Recommendation {
  image_url: String,
  #[serde(rename = "mal_id")]
  id: u32,
  recommendation_count: u32,
  recommendation_url: String,
  title: String,
  url: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Recommendations {
  recommendations: Vec<Recommendation>,
}

impl Recommendations {
  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, Box<dyn Error>> {
    let response = client
      .get::<Self>(&format!("/anime/{}/recommendations", id))
      .await?;
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
  async fn can_get_recommendations_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(format!("/anime/{}/recommendations", id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/recommendations_{}.json",
            id
          )));
      });

      let recommendations = Recommendations::from_id(&client, id).await;
      mock.assert();
      assert!(recommendations.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_recommendations_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mock = server.mock(|when, then| {
        when.path(format!("/anime/{}/recommendations", id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/recommendations_{}.json",
            id
          )));
      });

      let recommendations = Recommendations::from_id(&client, id).await;
      mock.assert();
      assert!(
        recommendations.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );
    }

    Ok(())
  }
}

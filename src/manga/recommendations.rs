use super::super::common::error::JikanError;
use super::super::common::recommendations::Recommendation;
use super::super::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Recommendations {
  recommendations: Vec<Recommendation>,
}

impl Recommendations {
  pub fn get_url_path(id: u32) -> String {
    format!("/manga/{}/recommendations", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client
      .get::<Self>(&Recommendations::get_url_path(id))
      .await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::super::utils::test_helper as utils_test_helper;
  use super::super::test_helper::{self, MangaTestSuite};
  use super::*;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_recommendations_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_valid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Recommendations::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/recommendations_{}.json",
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

    for MangaTestSuite { id, name } in test_helper::get_invalid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(Recommendations::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/recommendations_{}.json",
            id
          )));
      });

      let recommendations = Recommendations::from_id(&client, id).await;
      mock.assert();
      assert!(
        recommendations.is_err(),
        "Response for manga \"{}\" is not 404",
        name,
      );
    }

    Ok(())
  }
}

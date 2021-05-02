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
  use super::super::{test_helper, test_helper::AnimeTestSuite};
  use super::*;
  use serial_test::serial;
  use std::error::Error;
  use std::thread;

  #[tokio::test]
  #[serial]
  async fn can_get_recommendations_by_id() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let recommendations = Recommendations::from_id(&client, id).await;

      match recommendations {
        Ok(_) => assert!(recommendations.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[tokio::test]
  #[serial]
  async fn can_handle_recommendations_404() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      assert!(
        Recommendations::from_id(&client, id).await.is_err(),
        "Response for anime \"{}\" is not 404",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }
}

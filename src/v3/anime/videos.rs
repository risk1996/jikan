use crate::v3::common::error::JikanError;
use crate::v3::common::request::RequestMetadata;
use crate::v3::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct PromoVideo {
  image_url: Url,
  title: String,
  video_url: Url,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct EpisodeVideo {
  episode: String,
  image_url: Url,
  title: String,
  url: Url,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Videos {
  #[serde(flatten)]
  metadata: RequestMetadata,
  promo: Vec<PromoVideo>,
  episodes: Vec<EpisodeVideo>,
}

impl Videos {
  pub fn get_url_path(id: u32) -> String {
    format!("/anime/{}/videos", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&Videos::get_url_path(id)).await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::test_helper::{self, AnimeTestSuite};
  use super::*;
  use crate::v3::utils::test_helper as utils_test_helper;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_videos_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(Videos::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/v3/anime/__test__/videos_{}.json",
            id
          )));
      });

      let videos = Videos::from_id(&client, id).await;
      mock.assert();
      assert!(videos.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_videos_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mock = server.mock(|when, then| {
        when.path(Videos::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/v3/anime/__test__/videos_{}.json",
            id
          )));
      });

      let video = Videos::from_id(&client, id).await;
      mock.assert();
      assert!(video.is_err(), "Response for anime \"{}\" is not 404", name,);
    }

    Ok(())
  }
}

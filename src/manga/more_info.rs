use crate::common::error::JikanError;
use crate::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct MoreInfo {
  #[serde(rename = "moreinfo")]
  more_info: Option<String>,
}

impl MoreInfo {
  pub fn get_url_path(id: u32) -> String {
    format!("/manga/{}/moreinfo", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, JikanError> {
    let response = client.get::<Self>(&MoreInfo::get_url_path(id)).await?;
    Ok(response.into_body())
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
  async fn can_get_more_info_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_valid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(MoreInfo::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/more_info_{}.json",
            id
          )));
      });

      let more_info = MoreInfo::from_id(&client, id).await;
      mock.assert();
      assert!(more_info.is_ok(), "{}", name);
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_more_info_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for MangaTestSuite { id, name } in test_helper::get_invalid_mangas() {
      let mock = server.mock(|when, then| {
        when.path(MoreInfo::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/manga/__test__/more_info_{}.json",
            id
          )));
      });

      let more_info = MoreInfo::from_id(&client, id).await;
      mock.assert();
      assert!(
        more_info?.more_info.is_none(),
        "Response for manga \"{}\" is not 404",
        name,
      );
    }

    Ok(())
  }
}

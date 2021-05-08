extern crate hyper;
use super::super::common::error::JikanError;
use hyper::{body, client::HttpConnector, Body, Client, Response, Uri};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

static BASE_URL: &'static str = "https://api.jikan.moe/v3";

pub struct JikanHttpClient {
  base_url: String,
  client: Client<HttpsConnector<HttpConnector>>,
}

impl JikanHttpClient {
  pub fn new(base_url: &str) -> Self {
    let base_url = String::from(base_url);
    let mut https = HttpsConnector::new();
    https.https_only(base_url.starts_with("https"));
    let client = Client::builder().build::<_, Body>(https);
    JikanHttpClient { base_url, client }
  }

  pub fn default() -> Self {
    JikanHttpClient::new(BASE_URL)
  }

  pub async fn get<T>(&self, path: &str) -> Result<Response<T>, JikanError>
  where
    for<'de> T: Deserialize<'de>,
  {
    let url: Uri = format!("{}{}", self.base_url, path).parse()?;

    let response = self.client.get(url).await?;
    let (parts, body) = response.into_parts();
    let body = &body::to_bytes(body).await?;
    let body = serde_json::from_slice(body)?;

    Ok(Response::from_parts(parts, body))
  }
}

#[cfg(test)]
mod tests {
  use super::super::test_helper;
  use super::*;
  use derive_getters::Getters;
  use httpmock::Method::GET;
  use httpmock::MockServer;
  use serde::{Deserialize, Serialize};
  use std::error::Error;

  #[derive(Deserialize, Getters, Serialize)]
  struct About {
    #[serde(rename = "Website")]
    website: String,
  }

  #[tokio::test]
  async fn can_call_base_url() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
      when.method(GET).path("/");
      then.status(200).body(test_helper::file_to_string(
        "src/utils/__test__/base_url.json",
      ));
    });

    let client = JikanHttpClient::new(&server.base_url());
    let response_body = client.get::<About>("").await?.into_body();

    mock.assert();
    assert!(response_body.website().contains("jikan.moe"));

    Ok(())
  }
}

extern crate hyper;
use hyper::{body, client::HttpConnector, Body, Client, Response, Uri};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::error::Error;

static BASE_URL: &'static str = "https://api.jikan.moe/v3";

pub struct JikanHttpClient {
  client: Client<HttpsConnector<HttpConnector>>,
}

impl JikanHttpClient {
  pub fn new() -> Self {
    let mut https = HttpsConnector::new();
    https.https_only(true);

    let client = Client::builder().build::<_, Body>(https);

    JikanHttpClient { client }
  }

  pub async fn get<T>(&self, path: &str) -> Result<Response<T>, Box<dyn Error>>
  where
    for<'de> T: Deserialize<'de>,
  {
    let url: Uri = format!("{}{}", &BASE_URL, path).parse()?;

    let response = self.client.get(url).await?;
    let (parts, body) = response.into_parts();
    let body = &body::to_bytes(body).await?;
    let body = serde_json::from_slice(body)?;

    Ok(Response::from_parts(parts, body))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use derive_getters::Getters;
  use serde::{Deserialize, Serialize};

  #[derive(Deserialize, Getters, Serialize)]
  struct About {
    #[serde(rename = "Website")]
    website: String,
  }

  #[tokio::test]
  async fn can_call_base_url() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();
    let response_body = client.get::<About>("").await?.into_body();
    assert!(response_body.website().contains("jikan.moe"));

    Ok(())
  }
}

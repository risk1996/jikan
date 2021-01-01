use reqwest::{Client, RequestBuilder, Url};

lazy_static! {
  static ref BASE_URL: Url = Url::parse("https://api.jikan.moe/v3/").unwrap();
}

pub struct JikanHttpClient {
  client: Client,
}

impl JikanHttpClient {
  pub fn new() -> Self {
    let client = Client::builder().https_only(true).build().unwrap();
    JikanHttpClient { client }
  }

  pub fn get(&self, path: &str) -> RequestBuilder {
    let url = BASE_URL.clone();
    let url = url.join(path).expect("path parsing error");
    self.client.get(url)
  }

  pub fn post(&self, path: &str) -> RequestBuilder {
    let url = BASE_URL.clone();
    let url = url.join(path).expect("path parsing error");
    self.client.post(url)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[actix_rt::test]
  async fn can_call_base_url() {
    let client = JikanHttpClient::new();
    let response = client.get("").send().await;
    let body = response.unwrap();
    let body: &str = &body.text().await.unwrap()[..];
    assert!(!body.is_empty());
  }
}

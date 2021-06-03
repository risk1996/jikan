mod info;

pub use self::info::{Info, *};
use super::common::error::JikanError;
use super::utils::httpc::JikanHttpClient;

#[cfg(test)]
mod test_helper;

pub struct Manga {
  client: JikanHttpClient,
  id: u32,
}

// Used to get data under `/manga` from `jikan.moe`'s API
// Read more here: https://jikan.docs.apiary.io/#reference/0/manga
impl Manga {
  fn create(client: JikanHttpClient, id: u32) -> Self {
    Manga { client, id }
  }

  pub fn new(id: u32) -> Self {
    let client = JikanHttpClient::default();
    Manga::create(client, id)
  }

  pub async fn info(&self) -> Result<Info, JikanError> {
    Info::from_id(&self.client, self.id).await
  }
}

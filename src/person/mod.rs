mod info;
mod pictures;

pub use self::info::{Info, *};
pub use self::pictures::{Pictures, *};
use crate::common::error::JikanError;
pub use crate::common::{characters::*, pictures::*, staff::*};
use crate::utils::httpc::JikanHttpClient;

#[cfg(test)]
mod test_helper;

pub struct Person {
  client: JikanHttpClient,
  id: u32,
}

impl Person {
  fn create(client: JikanHttpClient, id: u32) -> Self {
    Person { client, id }
  }

  pub fn new(id: u32) -> Self {
    let client = JikanHttpClient::default();
    Person::create(client, id)
  }

  pub async fn info(&self) -> Result<Info, JikanError> {
    Info::from_id(&self.client, self.id).await
  }

  pub async fn pictures(&self) -> Result<Pictures, JikanError> {
    Pictures::from_id(&self.client, self.id).await
  }
}

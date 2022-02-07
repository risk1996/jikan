mod info;
mod pictures;

pub use self::info::{Info, *};
pub use self::pictures::{Pictures, *};
pub use crate::v3::common::{characters::*, pictures::*, staff::*};
use crate::v3::common::{error::JikanError, API_V3_BASE_URL};
use crate::v3::utils::httpc::JikanHttpClient;

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
    let client = JikanHttpClient::new(API_V3_BASE_URL);
    Person::create(client, id)
  }

  pub async fn info(&self) -> Result<Info, JikanError> {
    Info::from_id(&self.client, self.id).await
  }

  pub async fn pictures(&self) -> Result<Pictures, JikanError> {
    Pictures::from_id(&self.client, self.id).await
  }
}

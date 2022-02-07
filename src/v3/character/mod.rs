mod info;

pub use self::info::{Info, *};
pub use crate::v3::common::{characters::*, voice_actor::*};
use crate::v3::common::{error::JikanError, API_V3_BASE_URL};
use crate::v3::utils::httpc::JikanHttpClient;

#[cfg(test)]
mod test_helper;

pub struct Character {
  client: JikanHttpClient,
  id: u32,
}

impl Character {
  fn create(client: JikanHttpClient, id: u32) -> Self {
    Character { client, id }
  }

  pub fn new(id: u32) -> Self {
    let client = JikanHttpClient::new(API_V3_BASE_URL);
    Character::create(client, id)
  }

  pub async fn info(&self) -> Result<Info, JikanError> {
    Info::from_id(&self.client, self.id).await
  }
}

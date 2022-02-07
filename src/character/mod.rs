mod info;

pub use self::info::{Info, *};
use crate::common::error::JikanError;
pub use crate::common::{characters::*, voice_actor::*};
use crate::utils::httpc::JikanHttpClient;

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
    let client = JikanHttpClient::default();
    Character::create(client, id)
  }

  pub async fn info(&self) -> Result<Info, JikanError> {
    Info::from_id(&self.client, self.id).await
  }
}

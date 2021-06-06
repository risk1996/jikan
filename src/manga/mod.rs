mod characters;
mod forum;
mod info;
mod more_info;
mod news;
mod pictures;
mod recommendations;
mod reviews;
mod stats;
mod user_updates;

pub use self::characters::{Characters, *};
pub use self::forum::{Forum, *};
pub use self::info::{Info, *};
pub use self::more_info::{MoreInfo, *};
pub use self::news::{News, *};
pub use self::pictures::{Pictures, *};
pub use self::recommendations::{Recommendations, *};
pub use self::reviews::{Reviews, *};
pub use self::stats::{Stats, *};
pub use self::user_updates::{UserUpdates, *};
use super::common::error::JikanError;
pub use super::common::{
  forum::*, news::*, pictures::*, recommendations::*, stats::*, tag::*, user_updates::*,
};
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

  pub async fn characters(&self) -> Result<Characters, JikanError> {
    Characters::from_id(&self.client, self.id).await
  }

  pub async fn news(&self) -> Result<News, JikanError> {
    News::from_id(&self.client, self.id).await
  }

  pub async fn pictures(&self) -> Result<Pictures, JikanError> {
    Pictures::from_id(&self.client, self.id).await
  }

  pub async fn stats(&self) -> Result<Stats, JikanError> {
    Stats::from_id(&self.client, self.id).await
  }

  pub async fn forum(&self) -> Result<Forum, JikanError> {
    Forum::from_id(&self.client, self.id).await
  }

  pub async fn more_info(&self) -> Result<MoreInfo, JikanError> {
    MoreInfo::from_id(&self.client, self.id).await
  }

  pub async fn reviews_at_page(&self, page: u32) -> Result<Reviews, JikanError> {
    Reviews::from_id_at_page(&self.client, self.id, page).await
  }

  pub async fn recommendations(&self) -> Result<Recommendations, JikanError> {
    Recommendations::from_id(&self.client, self.id).await
  }

  pub async fn user_updates_at_page(&self, page: u32) -> Result<UserUpdates, JikanError> {
    UserUpdates::from_id_at_page(&self.client, self.id, page).await
  }
}

mod characters_staff;
mod episodes;
mod forum;
mod info;
mod more_info;
mod news;
mod pictures;
mod recommendations;
mod reviews;
mod stats;
mod user_updates;
mod videos;

pub use self::characters_staff::{CharactersStaff, *};
pub use self::episodes::{Episodes, *};
pub use self::forum::{Forum, *};
pub use self::info::{Info, *};
pub use self::more_info::MoreInfo;
pub use self::news::{News, *};
pub use self::pictures::{Pictures, *};
pub use self::recommendations::{Recommendations, *};
pub use self::reviews::{Reviews, *};
pub use self::stats::{Stats, *};
pub use self::user_updates::{UserUpdates, *};
pub use self::videos::{Videos, *};
pub use crate::v3::common::{
  characters::*, forum::*, news::*, pictures::*, recommendations::*, staff::*, stats::*, tag::*,
  user_updates::*, voice_actor::*,
};
use crate::v3::common::{error::JikanError, API_V3_BASE_URL};
use crate::v3::utils::httpc::JikanHttpClient;

#[cfg(test)]
mod test_helper;

pub struct Anime {
  client: JikanHttpClient,
  id: u32,
}

/// Used to get data under `/anime` from `jikan.moe`'s API
/// Read more here: https://jikan.docs.apiary.io/#reference/0/anime
impl Anime {
  fn create(client: JikanHttpClient, id: u32) -> Self {
    Anime { client, id }
  }

  /// Creates a `jikan::v3::anime::Anime` instance.
  ///
  /// # Example
  /// ```
  /// use jikan::v3::anime::Anime;
  ///
  /// fn get_naruto() -> Anime {
  ///   // Instantiate "Naruto" anime
  ///   Anime::new(20)
  /// }
  /// ```
  pub fn new(id: u32) -> Self {
    let client = JikanHttpClient::new(API_V3_BASE_URL);
    Anime::create(client, id)
  }

  /// Gets anime information.
  ///
  /// Reference: https://jikan.docs.apiary.io/#reference/0/anime
  ///
  /// # Example
  /// ```
  /// use jikan::v3::anime::{AiringStatus, Info, Anime};
  /// # use std::error::Error;
  ///
  /// # fn get_naruto() -> Anime {
  /// #   Anime::new(20)
  /// # }
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), Box<dyn Error>> {
  ///   // See Anime::new(...)
  ///   let naruto_info = get_naruto().info().await?;
  ///   assert_eq!(naruto_info.id(), &20u32);
  ///   assert_eq!(naruto_info.title(), "Naruto");
  ///   assert_eq!(naruto_info.is_airing(), &false);
  ///   assert_eq!(
  ///     naruto_info.airing_status(),
  ///     &AiringStatus::FinishedAiring,
  ///   );
  ///   Ok(())
  /// }
  /// ```
  pub async fn info(&self) -> Result<Info, JikanError> {
    Info::from_id(&self.client, self.id).await
  }

  pub async fn characters_staff(&self) -> Result<CharactersStaff, JikanError> {
    CharactersStaff::from_id(&self.client, self.id).await
  }

  pub async fn episodes_at_page(&self, page: u32) -> Result<Episodes, JikanError> {
    Episodes::from_id_at_page(&self.client, self.id, page).await
  }

  pub async fn news(&self) -> Result<News, JikanError> {
    News::from_id(&self.client, self.id).await
  }

  pub async fn pictures(&self) -> Result<Pictures, JikanError> {
    Pictures::from_id(&self.client, self.id).await
  }

  pub async fn videos(&self) -> Result<Videos, JikanError> {
    Videos::from_id(&self.client, self.id).await
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

pub use self::character_staff::{CharactersStaff, *};
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
use super::utils::httpc::JikanHttpClient;
use std::error::Error;

mod character_staff;
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

mod test_helper;

pub struct Anime {
  client: JikanHttpClient,
  id: u32,
}

/// Used to get data under `/anime` from `jikan.moe`'s API
/// Read more here: https://jikan.docs.apiary.io/#reference/0/anime
impl Anime {
  /// Creates a `jikan::anime::Anime` instance.
  ///
  /// # Example
  /// ```
  /// use jikan::anime::Anime;
  ///
  /// fn main() {
  ///   let anime = Anime::new(20);
  /// }
  /// ```
  pub fn new(id: u32) -> Self {
    let client = JikanHttpClient::new();
    Anime { client, id }
  }

  /// Gets anime information.
  ///
  /// Reference: https://jikan.docs.apiary.io/#reference/0/anime
  ///
  /// # Example
  /// ```
  /// use jikan::anime::{AiringStatus, Info, Anime};
  ///
  /// async fn get_naruto() -> Info {
  ///   let anime = Anime::new(20);
  ///   let naruto = anime.info().await.unwrap();
  ///
  ///   naruto
  /// }
  ///
  /// #[tokio::main]
  /// async fn main() {
  ///   let naruto = get_naruto().await;
  ///   assert_eq!(naruto.id(), &20u32);
  ///   assert_eq!(naruto.title(), "Naruto");
  ///   assert_eq!(naruto.is_airing(), &false);
  ///   assert_eq!(
  ///     naruto.airing_status(),
  ///     &AiringStatus::FinishedAiring,
  ///   );
  /// }
  /// ```
  pub async fn info(&self) -> Result<Info, Box<dyn Error>> {
    Info::from_id(&self.client, self.id).await
  }

  pub async fn characters_staff(&self) -> Result<CharactersStaff, Box<dyn Error>> {
    CharactersStaff::from_id(&self.client, self.id).await
  }

  pub async fn episodes_at_page(&self, page: u32) -> Result<Episodes, Box<dyn Error>> {
    Episodes::from_id_at_page(&self.client, self.id, page).await
  }

  pub async fn news(&self) -> Result<News, Box<dyn Error>> {
    News::from_id(&self.client, self.id).await
  }

  pub async fn pictures(&self) -> Result<Pictures, Box<dyn Error>> {
    Pictures::from_id(&self.client, self.id).await
  }

  pub async fn videos(&self) -> Result<Videos, Box<dyn Error>> {
    Videos::from_id(&self.client, self.id).await
  }

  pub async fn stats(&self) -> Result<Stats, Box<dyn Error>> {
    Stats::from_id(&self.client, self.id).await
  }

  pub async fn forum(&self) -> Result<Forum, Box<dyn Error>> {
    Forum::from_id(&self.client, self.id).await
  }

  pub async fn more_info(&self) -> Result<MoreInfo, Box<dyn Error>> {
    MoreInfo::from_id(&self.client, self.id).await
  }

  pub async fn reviews(&self, page: u32) -> Result<Reviews, Box<dyn Error>> {
    Reviews::from_id_at_page(&self.client, self.id, page).await
  }

  pub async fn recommendations(&self) -> Result<Recommendations, Box<dyn Error>> {
    Recommendations::from_id(&self.client, self.id).await
  }

  pub async fn user_updates(&self, page: u32) -> Result<UserUpdates, Box<dyn Error>> {
    UserUpdates::from_id_at_page(&self.client, self.id, page).await
  }
}

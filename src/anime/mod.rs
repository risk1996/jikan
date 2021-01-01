use self::character_staff::CharactersStaff;
use self::episodes::Episodes;
use self::info::Info;
use self::news::News;
use super::utils::httpc::JikanHttpClient;
use std::error::Error;

pub mod character_staff;
pub mod episodes;
pub mod info;
pub mod news;

mod test_helper;

pub struct Anime {
  client: JikanHttpClient,
}

/// Used to get data under `/anime` from `jikan.moe`'s API
/// Read more here: https://jikan.docs.apiary.io/#reference/0/anime
impl Anime {
  /// Creates a `jikan::anime::Anime` instance
  ///
  /// # Example
  /// ```
  /// use jikan::anime::Anime;
  ///
  /// fn main() {
  ///   let anime = Anime::new();
  /// }
  /// ```
  pub fn new() -> Self {
    let client = JikanHttpClient::new();
    Anime { client }
  }

  /// Gets information for certain anime
  ///
  /// # Example
  /// ```
  /// use jikan::anime::{info::{AiringStatus, Info}, Anime};
  ///
  /// async fn get_naruto() -> Info {
  ///   let anime = Anime::new();
  ///   let naruto = anime.get_info_by_id(20).await.unwrap();
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
  pub async fn get_info_by_id(&self, id: u32) -> Result<Info, Box<dyn Error>> {
    Info::from_id(&self.client, id).await
  }

  pub async fn get_characters_staff_by_id(
    &self,
    id: u32,
  ) -> Result<CharactersStaff, Box<dyn Error>> {
    CharactersStaff::from_id(&self.client, id).await
  }

  pub async fn get_episode_by_id_at_page(
    &self,
    id: u32,
    page: u32,
  ) -> Result<Episodes, Box<dyn Error>> {
    Episodes::from_id_at_page(&self.client, id, page).await
  }

  pub async fn get_news_by_id(&self, id: u32) -> Result<News, Box<dyn Error>> {
    News::from_id(&self.client, id).await
  }
}

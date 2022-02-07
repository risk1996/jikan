pub mod error;
pub mod request;

pub static API_V3_BASE_URL: &'static str = "https://api.jikan.moe/v3";

#[cfg(any(
  feature = "anime_v3",
  feature = "manga_v3",
  feature = "person_v3",
  feature = "character_v3",
))]
pub mod characters;

#[cfg(any(feature = "anime_v3", feature = "manga_v3"))]
pub mod forum;

#[cfg(any(feature = "anime_v3", feature = "manga_v3"))]
pub mod news;

#[cfg(any(feature = "anime_v3", feature = "manga_v3", feature = "person_v3"))]
pub mod pictures;

#[cfg(any(feature = "anime_v3", feature = "person_v3"))]
pub mod staff;

#[cfg(any(feature = "anime_v3", feature = "manga_v3"))]
pub mod stats;

#[cfg(any(feature = "anime_v3", feature = "manga_v3"))]
pub mod recommendations;

#[cfg(any(feature = "anime_v3", feature = "manga_v3"))]
pub mod tag;

#[cfg(any(feature = "anime_v3", feature = "manga_v3"))]
pub mod user_updates;

#[cfg(any(feature = "anime_v3", feature = "character_v3"))]
pub mod voice_actor;

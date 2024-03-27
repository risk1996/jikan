#[cfg(any(
  feature = "anime_v3",
  feature = "manga_v3",
  feature = "person_v3",
  feature = "character_v3",
))]
pub mod v3;

#[cfg(any(feature = "anime_v4",))]
pub mod v4;

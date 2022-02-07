mod common;
mod utils;

pub use common::*;
pub use utils::*;

#[cfg(feature = "anime_v3")]
pub mod anime;

#[cfg(feature = "manga_v3")]
pub mod manga;

#[cfg(feature = "person_v3")]
pub mod person;

#[cfg(feature = "character_v3")]
pub mod character;

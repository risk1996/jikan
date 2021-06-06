pub mod error;
pub mod request;

#[cfg(any(feature = "anime", feature = "manga", feature = "person"))]
pub mod characters;

#[cfg(any(feature = "anime", feature = "manga"))]
pub mod forum;

#[cfg(any(feature = "anime", feature = "manga"))]
pub mod news;

#[cfg(any(feature = "anime", feature = "manga", feature = "person"))]
pub mod pictures;

#[cfg(any(feature = "anime", feature = "person"))]
pub mod staff;

#[cfg(any(feature = "anime", feature = "manga"))]
pub mod stats;

#[cfg(any(feature = "anime", feature = "manga"))]
pub mod recommendations;

#[cfg(any(feature = "anime", feature = "manga"))]
pub mod tag;

#[cfg(any(feature = "anime", feature = "manga"))]
pub mod user_updates;

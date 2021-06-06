pub mod error;

#[cfg(feature = "anime")]
#[cfg(feature = "manga")]
pub mod forum;

#[cfg(feature = "anime")]
#[cfg(feature = "manga")]
pub mod news;

#[cfg(feature = "anime")]
#[cfg(feature = "manga")]
pub mod pictures;

#[cfg(feature = "anime")]
#[cfg(feature = "manga")]
pub mod stats;

#[cfg(feature = "anime")]
#[cfg(feature = "manga")]
pub mod recommendations;

#[cfg(feature = "anime")]
#[cfg(feature = "manga")]
pub mod tag;

#[cfg(feature = "anime")]
#[cfg(feature = "manga")]
pub mod user_updates;

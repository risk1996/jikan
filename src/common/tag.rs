use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum TagType {
  #[serde(alias = "anime")]
  Anime,
  #[serde(alias = "manga")]
  Manga,
  #[serde(alias = "people")]
  People,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Tag {
  #[serde(rename = "mal_id")]
  id: u32,
  r#type: TagType,
  name: String,
  url: Url,
}

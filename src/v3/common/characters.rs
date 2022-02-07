use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum CharacterRole {
  Main,
  Supporting,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct CharacterOf {
  #[serde(alias = "mal_id")]
  id: u32,
  image_url: Url,
  name: String,
  role: CharacterRole,
  url: Url,
}

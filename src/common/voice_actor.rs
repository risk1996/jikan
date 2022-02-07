use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum VoiceActorLanguage {
  Brazilian,
  English,
  French,
  German,
  Hebrew,
  Hungarian,
  Italian,
  Japanese,
  Korean,
  Spanish,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct VoiceActor {
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: Url,
  language: VoiceActorLanguage,
  name: String,
  url: Url,
}

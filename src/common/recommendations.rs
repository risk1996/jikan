use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Recommendation {
  image_url: String,
  #[serde(rename = "mal_id")]
  id: u32,
  recommendation_count: u32,
  recommendation_url: String,
  title: String,
  url: String,
}

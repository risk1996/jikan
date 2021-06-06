use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Recommendation {
  image_url: Url,
  #[serde(rename = "mal_id")]
  id: u32,
  recommendation_count: u32,
  recommendation_url: Url,
  title: String,
  url: Url,
}

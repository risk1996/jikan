use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct User<StatusT> {
  date: DateTime<FixedOffset>,
  episodes_seen: Option<u32>,
  episodes_total: Option<u32>,
  image_url: Url,
  score: Option<u8>,
  status: StatusT,
  url: Url,
  username: String,
}

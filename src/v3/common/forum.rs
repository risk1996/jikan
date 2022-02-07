use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Post {
  author_name: String,
  author_url: Url,
  date_posted: DateTime<FixedOffset>,
  url: Url,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Topic {
  author_name: String,
  author_url: Url,
  date_posted: DateTime<FixedOffset>,
  #[serde(rename = "topic_id")]
  id: u32,
  last_post: Post,
  replies: u32,
  title: String,
  url: Url,
}

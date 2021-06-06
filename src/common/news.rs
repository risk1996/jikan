use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use url::Url;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct NewsArticle {
  author_name: String,
  author_url: Url,
  comments: u32,
  date: DateTime<FixedOffset>,
  forum_url: Url,
  image_url: Option<String>,
  intro: String,
  title: String,
  url: Url,
}

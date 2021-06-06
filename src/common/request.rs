use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct RequestMetadata {
  request_cache_expiry: u32,
  request_cached: bool,
  request_hash: String,
}

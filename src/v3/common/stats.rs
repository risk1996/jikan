use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Score {
  votes: u32,
  percentage: f32,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Scores {
  #[serde(alias = "1")]
  score_1: Score,
  #[serde(alias = "2")]
  score_2: Score,
  #[serde(alias = "3")]
  score_3: Score,
  #[serde(alias = "4")]
  score_4: Score,
  #[serde(alias = "5")]
  score_5: Score,
  #[serde(alias = "6")]
  score_6: Score,
  #[serde(alias = "7")]
  score_7: Score,
  #[serde(alias = "8")]
  score_8: Score,
  #[serde(alias = "9")]
  score_9: Score,
  #[serde(alias = "10")]
  score_10: Score,
}

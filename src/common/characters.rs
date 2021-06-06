use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum CharacterRole {
  Main,
  Supporting,
  #[serde(other)]
  Unknown,
}

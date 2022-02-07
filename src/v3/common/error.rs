use hyper::{http::uri::InvalidUri, Error as HttpError};
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JikanError {
  #[error("invalid uri")]
  InvalidUri(#[from] InvalidUri),
  #[error("http error")]
  HttpError(#[from] HttpError),
  #[error("serialization/deserialization error")]
  SerializationDeserializationError(#[from] SerdeError),
  #[error("unknown error")]
  Fail,
}

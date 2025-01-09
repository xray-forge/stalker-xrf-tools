use crate::error::chunk_error::ChunkError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Parsing of chunk error.
#[derive(Clone, Debug, PartialEq)]
pub struct ChunkInvalidError {
  pub message: String,
}

impl ChunkInvalidError {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self {
      message: message.into(),
    }
  }

  pub fn new_chunk_error<T>(message: T) -> ChunkError
  where
    T: Into<String>,
  {
    ChunkError::Invalid(Self {
      message: message.into(),
    })
  }
}

impl Display for ChunkInvalidError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "Invalid chunk error: {}", self.message)
  }
}

impl Error for ChunkInvalidError {}

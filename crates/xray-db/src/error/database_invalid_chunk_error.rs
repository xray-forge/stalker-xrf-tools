use crate::error::database_error::DatabaseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Parsing of chunk error.
#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseInvalidChunkError {
  pub message: String,
}

impl DatabaseInvalidChunkError {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self {
      message: message.into(),
    }
  }

  pub fn new_database_error<T>(message: T) -> DatabaseError
  where
    T: Into<String>,
  {
    DatabaseError::InvalidChunk(Self {
      message: message.into(),
    })
  }
}

impl Display for DatabaseInvalidChunkError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "Invalid chunk error: {}", self.message)
  }
}

impl Error for DatabaseInvalidChunkError {}

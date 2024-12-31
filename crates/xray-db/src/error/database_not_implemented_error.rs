use crate::error::database_error::DatabaseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Parsing error.
#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseNotImplementedError {
  pub message: String,
}

impl DatabaseNotImplementedError {
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
    DatabaseError::NotImplemented(Self {
      message: message.into(),
    })
  }
}

impl Display for DatabaseNotImplementedError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(
      formatter,
      "Database not implemented error: {}",
      self.message
    )
  }
}

impl Error for DatabaseNotImplementedError {}

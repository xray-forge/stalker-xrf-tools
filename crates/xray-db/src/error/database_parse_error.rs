use crate::error::database_error::DatabaseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Parsing error.
#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseParseError {
  pub message: String,
}

impl DatabaseParseError {
  pub fn new<T>(message: T) -> DatabaseParseError
  where
    T: Into<String>,
  {
    DatabaseParseError {
      message: message.into(),
    }
  }

  pub fn new_database_error<T>(message: T) -> DatabaseError
  where
    T: Into<String>,
  {
    DatabaseError::Parse(DatabaseParseError {
      message: message.into(),
    })
  }
}

impl Display for DatabaseParseError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(
      formatter,
      "Invalid data for parsing error: {}",
      self.message
    )
  }
}

impl Error for DatabaseParseError {}

use crate::error::archive_error::ArchiveError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Reading archive error.
#[derive(Debug)]
pub struct ArchiveReadError {
  pub message: String,
}

impl ArchiveReadError {
  pub fn new<T>(message: T) -> ArchiveReadError
  where
    T: Into<String>,
  {
    ArchiveReadError {
      message: message.into(),
    }
  }

  pub fn new_archive_error<T>(message: T) -> ArchiveError
  where
    T: Into<String>,
  {
    ArchiveError::Read(ArchiveReadError {
      message: message.into(),
    })
  }
}

impl Display for ArchiveReadError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}", self.message)
  }
}

impl Error for ArchiveReadError {}

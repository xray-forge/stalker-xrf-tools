use crate::error::archive_error::ArchiveError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

pub const ARCHIVE_READ_ERROR_GENERIC: &str = "GENERIC";
pub const ARCHIVE_READ_ERROR_NOT_FOUND: &str = "NOT_FOUND";
pub const ARCHIVE_READ_ERROR_INVALID_FORMAT: &str = "INVALID_FORMAT";

/// Reading archive error.
#[derive(Debug)]
pub struct ArchiveReadError {
  pub code: String,
  pub message: String,
}

impl ArchiveReadError {
  pub fn new<T>(message: T) -> ArchiveReadError
  where
    T: Into<String>,
  {
    ArchiveReadError {
      code: ARCHIVE_READ_ERROR_GENERIC.into(),
      message: message.into(),
    }
  }

  pub fn new_with_code<C, T>(code: C, message: T) -> ArchiveReadError
  where
    C: Into<String>,
    T: Into<String>,
  {
    ArchiveReadError {
      code: code.into(),
      message: message.into(),
    }
  }

  pub fn new_archive_error<T>(message: T) -> ArchiveError
  where
    T: Into<String>,
  {
    ArchiveError::Read(ArchiveReadError {
      code: ARCHIVE_READ_ERROR_GENERIC.into(),
      message: message.into(),
    })
  }

  pub fn new_archive_error_with_code<C, T>(code: C, message: T) -> ArchiveError
  where
    C: Into<String>,
    T: Into<String>,
  {
    ArchiveError::Read(ArchiveReadError {
      code: code.into(),
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

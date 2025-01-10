use std::io::Error as IoError;
use thiserror::Error as ThisError;

pub const ARCHIVE_READ_ERROR_GENERIC: &str = "GENERIC";
pub const ARCHIVE_READ_ERROR_NOT_FOUND: &str = "NOT_FOUND";
pub const ARCHIVE_READ_ERROR_INVALID_FORMAT: &str = "INVALID_FORMAT";

/// Error while working with archive files.
#[derive(ThisError, Debug)]
pub enum ArchiveError {
  #[error("Archive IO error: {0}")]
  Io(#[from] IoError),
  #[error("Archive read error: {message:?}")]
  Read { message: String, code: String },
}

impl ArchiveError {
  pub fn new_read_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Read {
      code: ARCHIVE_READ_ERROR_GENERIC.into(),
      message: message.into(),
    }
  }

  pub fn new_read_error_with_code<C, T>(code: C, message: T) -> Self
  where
    C: Into<String>,
    T: Into<String>,
  {
    Self::Read {
      code: code.into(),
      message: message.into(),
    }
  }
}

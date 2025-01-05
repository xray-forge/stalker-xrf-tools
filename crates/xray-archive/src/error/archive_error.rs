use crate::error::archive_read_error::ArchiveReadError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io::Error as IoError;

/// Error while working with archive files.
#[derive(Debug)]
pub enum ArchiveError {
  Io(IoError),
  Read(ArchiveReadError),
}

impl Display for ArchiveError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      Self::Io(ref error) => error.fmt(formatter),
      Self::Read(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for ArchiveError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::Read(ref error) => error.source(),
      Self::Io(ref error) => error.source(),
    }
  }
}

impl From<IoError> for ArchiveError {
  fn from(error: IoError) -> Self {
    Self::Io(error)
  }
}

impl From<ArchiveReadError> for ArchiveError {
  fn from(error: ArchiveReadError) -> Self {
    Self::Read(error)
  }
}

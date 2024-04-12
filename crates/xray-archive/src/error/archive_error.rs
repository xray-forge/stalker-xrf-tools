use crate::error::archive_read_error::ArchiveReadError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

/// Error while working with archive files.
#[derive(Debug)]
pub enum ArchiveError {
  Io(io::Error),
  Read(ArchiveReadError),
}

impl Display for ArchiveError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      ArchiveError::Io(ref error) => error.fmt(formatter),
      ArchiveError::Read(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for ArchiveError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      ArchiveError::Read(ref error) => error.source(),
      ArchiveError::Io(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for ArchiveError {
  fn from(error: io::Error) -> Self {
    ArchiveError::Io(error)
  }
}

impl From<ArchiveReadError> for ArchiveError {
  fn from(error: ArchiveReadError) -> Self {
    ArchiveError::Read(error)
  }
}

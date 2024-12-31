use crate::error::database_invalid_chunk_error::DatabaseInvalidChunkError;
use crate::error::database_not_implemented_error::DatabaseNotImplementedError;
use crate::error::database_parse_error::DatabaseParseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;
use xray_ltx::LtxError;

/// Error while working with DB data parsing/reading/writing/importing/exporting.
#[derive(Debug)]
pub enum DatabaseError {
  Io(io::Error),
  InvalidChunk(DatabaseInvalidChunkError),
  NotImplemented(DatabaseNotImplementedError),
  Parse(DatabaseParseError),
  LtxError(LtxError),
  Generic(Box<dyn Error>),
}

impl Display for DatabaseError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      Self::Io(ref error) => error.fmt(formatter),
      Self::InvalidChunk(ref error) => error.fmt(formatter),
      Self::Parse(ref error) => error.fmt(formatter),
      Self::NotImplemented(ref error) => error.fmt(formatter),
      Self::LtxError(ref error) => error.fmt(formatter),
      Self::Generic(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for DatabaseError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::Io(ref error) => error.source(),
      Self::InvalidChunk(ref error) => error.source(),
      Self::Parse(ref error) => error.source(),
      Self::LtxError(ref error) => error.source(),
      Self::NotImplemented(ref error) => error.source(),
      Self::Generic(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for DatabaseError {
  fn from(error: io::Error) -> Self {
    Self::Io(error)
  }
}

impl From<DatabaseInvalidChunkError> for DatabaseError {
  fn from(error: DatabaseInvalidChunkError) -> Self {
    Self::InvalidChunk(error)
  }
}

impl From<DatabaseNotImplementedError> for DatabaseError {
  fn from(error: DatabaseNotImplementedError) -> Self {
    Self::NotImplemented(error)
  }
}

impl From<DatabaseParseError> for DatabaseError {
  fn from(error: DatabaseParseError) -> Self {
    Self::Parse(error)
  }
}

impl From<LtxError> for DatabaseError {
  fn from(error: LtxError) -> Self {
    Self::LtxError(error)
  }
}

impl From<Box<dyn Error>> for DatabaseError {
  fn from(error: Box<dyn Error>) -> Self {
    Self::Generic(error)
  }
}

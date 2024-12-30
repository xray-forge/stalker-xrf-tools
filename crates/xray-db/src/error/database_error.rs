use crate::error::database_invalid_chunk_error::DatabaseInvalidChunkError;
use crate::error::database_not_implemented_error::DatabaseNotImplementedError;
use crate::error::database_parse_error::DatabaseParseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

/// Error while working with DB data parsing/reading/writing/importing/exporting.
#[derive(Debug)]
pub enum DatabaseError {
  Io(io::Error),
  InvalidChunk(DatabaseInvalidChunkError),
  NotImplemented(DatabaseNotImplementedError),
  Parse(DatabaseParseError),
  Generic(Box<dyn Error>),
}

impl Display for DatabaseError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      DatabaseError::Io(ref error) => error.fmt(formatter),
      DatabaseError::InvalidChunk(ref error) => error.fmt(formatter),
      DatabaseError::Parse(ref error) => error.fmt(formatter),
      DatabaseError::NotImplemented(ref error) => error.fmt(formatter),
      DatabaseError::Generic(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for DatabaseError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      DatabaseError::Io(ref error) => error.source(),
      DatabaseError::InvalidChunk(ref error) => error.source(),
      DatabaseError::Parse(ref error) => error.source(),
      DatabaseError::NotImplemented(ref error) => error.source(),
      DatabaseError::Generic(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for DatabaseError {
  fn from(error: io::Error) -> Self {
    DatabaseError::Io(error)
  }
}

impl From<DatabaseInvalidChunkError> for DatabaseError {
  fn from(error: DatabaseInvalidChunkError) -> Self {
    DatabaseError::InvalidChunk(error)
  }
}

impl From<DatabaseNotImplementedError> for DatabaseError {
  fn from(error: DatabaseNotImplementedError) -> Self {
    DatabaseError::NotImplemented(error)
  }
}

impl From<DatabaseParseError> for DatabaseError {
  fn from(error: DatabaseParseError) -> Self {
    DatabaseError::Parse(error)
  }
}

impl From<Box<dyn Error>> for DatabaseError {
  fn from(error: Box<dyn Error>) -> Self {
    DatabaseError::Generic(error)
  }
}

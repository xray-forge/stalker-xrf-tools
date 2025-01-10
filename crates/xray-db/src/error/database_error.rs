use crate::error::database_not_implemented_error::DatabaseNotImplementedError;
use crate::error::database_parse_error::DatabaseParseError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;
use xray_chunk::ChunkError;
use xray_ltx::LtxError;

/// Error while working with DB data parsing/reading/writing/importing/exporting.
#[derive(Debug)]
pub enum DatabaseError {
  Io(io::Error),
  Chunk(ChunkError),
  NotImplemented(DatabaseNotImplementedError),
  Parse(DatabaseParseError),
  Ltx(LtxError),
  Generic(Box<dyn Error>),
}

impl Display for DatabaseError {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    match *self {
      Self::Io(ref error) => error.fmt(formatter),
      Self::Chunk(ref error) => error.fmt(formatter),
      Self::Parse(ref error) => error.fmt(formatter),
      Self::NotImplemented(ref error) => error.fmt(formatter),
      Self::Ltx(ref error) => error.fmt(formatter),
      Self::Generic(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for DatabaseError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::Io(ref error) => error.source(),
      Self::Chunk(ref error) => error.source(),
      Self::Parse(ref error) => error.source(),
      Self::Ltx(ref error) => error.source(),
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

impl From<ChunkError> for DatabaseError {
  fn from(error: ChunkError) -> Self {
    Self::Chunk(error)
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
    Self::Ltx(error)
  }
}

impl From<Box<dyn Error>> for DatabaseError {
  fn from(error: Box<dyn Error>) -> Self {
    Self::Generic(error)
  }
}

impl Into<ChunkError> for DatabaseError {
  fn into(self) -> ChunkError {
    ChunkError::Generic(Box::new(self))
  }
}

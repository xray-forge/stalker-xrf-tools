use crate::error::chunk_parsing_error::ChunkParsingError;
use crate::error::invalid_chunk_error::ChunkInvalidError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

/// Error while working with DB data parsing/reading/writing/importing/exporting.
#[derive(Debug)]
pub enum ChunkError {
  Io(io::Error),
  Invalid(ChunkInvalidError),
  Parsing(ChunkParsingError),
  Generic(Box<dyn Error>),
}

impl Display for ChunkError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      Self::Io(ref error) => error.fmt(formatter),
      Self::Invalid(ref error) => error.fmt(formatter),
      Self::Parsing(ref error) => error.fmt(formatter),
      Self::Generic(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for ChunkError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::Io(ref error) => error.source(),
      Self::Invalid(ref error) => error.source(),
      Self::Parsing(ref error) => error.source(),
      Self::Generic(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for ChunkError {
  fn from(error: io::Error) -> Self {
    Self::Io(error)
  }
}

impl From<ChunkParsingError> for ChunkError {
  fn from(error: ChunkParsingError) -> Self {
    Self::Parsing(error)
  }
}

impl From<ChunkInvalidError> for ChunkError {
  fn from(error: ChunkInvalidError) -> Self {
    Self::Invalid(error)
  }
}

impl From<Box<dyn Error>> for ChunkError {
  fn from(error: Box<dyn Error>) -> Self {
    Self::Generic(error)
  }
}

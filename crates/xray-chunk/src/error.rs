use std::error::Error;
use std::io::Error as IoError;
use thiserror::Error as ThisError;

/// Error while working with DB data parsing/reading/writing/importing/exporting.
#[derive(ThisError, Debug)]
pub enum ChunkError {
  #[error("Chunk invalid error: {message:?}")]
  Invalid { message: String },
  #[error("Chunk parsing error: {message:?}")]
  Parsing { message: String },
  #[error("Ltx read error: {0}")]
  Generic(#[from] Box<dyn Error>),
  #[error("Ltx IO error: {0}")]
  Io(#[from] IoError),
}

impl ChunkError {
  pub fn new_invalid_chunk_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Invalid {
      message: message.into(),
    }
  }

  pub fn new_parsing_chunk_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Parsing {
      message: message.into(),
    }
  }
}

use std::error::Error;
use std::io;
use thiserror::Error as ThisError;
use xray_chunk::ChunkError;
use xray_ltx::LtxError;

/// Error while working with DB data parsing/reading/writing/importing/exporting.
#[derive(ThisError, Debug)]
pub enum DatabaseError {
  #[error("Database not implemented error: {message:?}")]
  NotImplemented { message: String },
  #[error("Database parse error: {message:?}")]
  Parse { message: String },
  #[error("Database chunk error: {0}")]
  Chunk(#[from] ChunkError),
  #[error("Database generic error: {0}")]
  Generic(#[from] Box<dyn Error>),
  #[error("Database LTX error: {0}")]
  Ltx(#[from] LtxError),
  #[error("Database IO error: {0}")]
  Io(#[from] io::Error),
}

impl DatabaseError {
  pub fn new_not_implemented_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::NotImplemented {
      message: message.into(),
    }
  }

  pub fn new_parse_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Parse {
      message: message.into(),
    }
  }
}

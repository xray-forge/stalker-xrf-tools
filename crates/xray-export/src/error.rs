use std::io;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ExportError {
  #[error("Export IO error: {0}")]
  Io(#[from] io::Error),
  #[error("Export parse error: {message:?}")]
  Parse { message: String },
}

impl ExportError {
  pub fn new_parse_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Parse {
      message: message.into(),
    }
  }
}

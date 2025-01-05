use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io::Error as IoError;

/// Error while working with translation file
#[derive(Debug)]
pub enum TranslationError {
  Io(IoError),
  UnknownLanguage(String),
  InvalidSourceJson(String),
}

impl Display for TranslationError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      Self::Io(ref error) => error.fmt(formatter),
      Self::UnknownLanguage(ref error) => error.fmt(formatter),
      Self::InvalidSourceJson(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for TranslationError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::Io(ref error) => error.source(),
      Self::UnknownLanguage(ref _error) => None,
      Self::InvalidSourceJson(ref _error) => None,
    }
  }
}

impl From<IoError> for TranslationError {
  fn from(error: IoError) -> Self {
    Self::Io(error)
  }
}

use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

/// Error while working with translation file
#[derive(Debug)]
pub enum TranslationError {
  Io(io::Error),
  UnknownLanguage(String),
  InvalidSourceJson(String),
}

impl Display for TranslationError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      TranslationError::Io(ref error) => error.fmt(formatter),
      TranslationError::UnknownLanguage(ref error) => error.fmt(formatter),
      TranslationError::InvalidSourceJson(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for TranslationError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      TranslationError::Io(ref error) => error.source(),
      TranslationError::UnknownLanguage(ref _error) => None,
      TranslationError::InvalidSourceJson(ref _error) => None,
    }
  }
}

impl From<io::Error> for TranslationError {
  fn from(err: io::Error) -> Self {
    TranslationError::Io(err)
  }
}

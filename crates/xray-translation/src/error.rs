use std::io::Error as IoError;
use thiserror::Error as ThisError;

/// Error while working with translation file
#[derive(ThisError, Debug)]
pub enum TranslationError {
  #[error("Translation: {message:?}")]
  UnknownLanguage { message: String },
  #[error("Translation: {message:?}")]
  InvalidSourceJson { message: String },
  #[error("Database IO error: {0}")]
  Io(#[from] IoError),
}

impl TranslationError {
  pub fn new_unknown_language_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::UnknownLanguage {
      message: message.into(),
    }
  }

  pub fn new_invalid_source_json_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::InvalidSourceJson {
      message: message.into(),
    }
  }
}

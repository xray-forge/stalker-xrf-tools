use crate::error::export_error::ExportError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ExportParseError {
  pub message: String,
}

impl ExportParseError {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self {
      message: message.into(),
    }
  }

  pub fn new_export_error<T>(message: T) -> ExportError
  where
    T: Into<String>,
  {
    ExportError::Parse(Self {
      message: message.into(),
    })
  }
}

impl Display for ExportParseError {
  fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
    write!(formatter, "{}", self.message)
  }
}

impl Error for ExportParseError {}

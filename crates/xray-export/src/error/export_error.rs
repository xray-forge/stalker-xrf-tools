use crate::error::parse_error::ExportParseError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum ExportError {
  Io(io::Error),
  Parse(ExportParseError),
}

impl Display for ExportError {
  fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
    match *self {
      ExportError::Io(ref error) => error.fmt(formatter),
      ExportError::Parse(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for ExportError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      ExportError::Io(ref error) => error.source(),
      ExportError::Parse(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for ExportError {
  fn from(err: io::Error) -> Self {
    ExportError::Io(err)
  }
}

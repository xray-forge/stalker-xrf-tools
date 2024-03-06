use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

/// Error while parsing an LTX document.
#[derive(Debug)]
pub enum LtxError {
  Io(io::Error),
  Parse(LtxParseError),
  Convert(LtxConvertError),
}

/// Parse error.
#[derive(Debug)]
pub struct LtxParseError {
  pub line: usize,
  pub col: usize,
  pub message: String,
}

/// Convert error.
#[derive(Debug)]
pub struct LtxConvertError {
  pub message: String,
}

impl LtxConvertError {
  pub fn new_ltx_error<T>(message: T) -> LtxError
  where
    T: Into<String>,
  {
    LtxError::Convert(LtxConvertError {
      message: message.into(),
    })
  }
}

impl Display for LtxParseError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}:{} {}", self.line, self.col, self.message)
  }
}

impl Display for LtxConvertError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}", self.message)
  }
}

impl Error for LtxParseError {}

impl Error for LtxConvertError {}

impl Display for LtxError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      LtxError::Io(ref error) => error.fmt(formatter),
      LtxError::Parse(ref error) => error.fmt(formatter),
      LtxError::Convert(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for LtxError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      LtxError::Io(ref error) => error.source(),
      LtxError::Parse(ref error) => error.source(),
      LtxError::Convert(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for LtxError {
  fn from(err: io::Error) -> Self {
    LtxError::Io(err)
  }
}

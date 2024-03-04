use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

/// Error while parsing an LTX document.
#[derive(Debug)]
pub enum LtxError {
  Io(io::Error),
  Parse(LtxParseError),
}

/// Parse error.
#[derive(Debug)]
pub struct LtxParseError {
  pub line: usize,
  pub col: usize,
  pub msg: String,
}

impl Display for LtxParseError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}:{} {}", self.line, self.col, self.msg)
  }
}

impl Error for LtxParseError {}

impl Display for LtxError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    match *self {
      LtxError::Io(ref err) => err.fmt(formatter),
      LtxError::Parse(ref err) => err.fmt(formatter),
    }
  }
}

impl Error for LtxError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      LtxError::Io(ref error) => error.source(),
      LtxError::Parse(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for LtxError {
  fn from(err: io::Error) -> Self {
    LtxError::Io(err)
  }
}

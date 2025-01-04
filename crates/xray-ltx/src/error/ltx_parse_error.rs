use crate::LtxError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Parsing error.
#[derive(Debug)]
pub struct LtxParseError {
  pub line: usize,
  pub col: usize,
  pub message: String,
}

impl LtxParseError {
  pub fn new<T>(line: usize, col: usize, message: T) -> Self
  where
    T: Into<String>,
  {
    Self {
      line,
      col,
      message: message.into(),
    }
  }

  pub fn new_ltx_error<T>(line: usize, col: usize, message: T) -> LtxError
  where
    T: Into<String>,
  {
    LtxError::Parse(Self::new(line, col, message))
  }
}

impl Display for LtxParseError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}:{} {}", self.line, self.col, self.message)
  }
}

impl Error for LtxParseError {}

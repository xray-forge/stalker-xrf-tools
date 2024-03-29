use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Parsing error.
#[derive(Debug)]
pub struct LtxParseError {
  pub line: usize,
  pub col: usize,
  pub message: String,
}

impl Display for LtxParseError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}:{} {}", self.line, self.col, self.message)
  }
}

impl Error for LtxParseError {}

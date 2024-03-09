use crate::error::ltx_error::LtxError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Reading LTX error.
#[derive(Debug)]
pub struct LtxReadError {
  pub message: String,
}

impl LtxReadError {
  pub fn new_ltx_error<T>(message: T) -> LtxError
  where
    T: Into<String>,
  {
    LtxError::Read(LtxReadError {
      message: message.into(),
    })
  }
}

impl Display for LtxReadError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}", self.message)
  }
}

impl Error for LtxReadError {}

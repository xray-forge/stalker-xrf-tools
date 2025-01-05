use crate::error::ltx_error::LtxError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

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
    LtxError::Convert(Self {
      message: message.into(),
    })
  }
}

impl Display for LtxConvertError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}", self.message)
  }
}

impl Error for LtxConvertError {}

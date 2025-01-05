use crate::error::ltx_error::LtxError;
use serde::Serialize;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Formatting LTX error.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LtxFormatError {
  pub message: String,
}

impl LtxFormatError {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self {
      message: message.into(),
    }
  }

  pub fn new_ltx_error<T>(message: T) -> LtxError
  where
    T: Into<String>,
  {
    LtxError::Format(Self {
      message: message.into(),
    })
  }
}

impl Display for LtxFormatError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}", self.message)
  }
}

impl Error for LtxFormatError {}

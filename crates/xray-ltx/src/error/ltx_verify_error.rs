use crate::error::ltx_error::LtxError;
use serde::Serialize;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Formatting LTX error.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LtxVerifyError {
  pub message: String,
}

impl LtxVerifyError {
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
    LtxError::Verify(Self {
      message: message.into(),
    })
  }
}

impl Display for LtxVerifyError {
  fn fmt(&self, formatter: &mut Formatter) -> Result {
    write!(formatter, "{}", self.message)
  }
}

impl Error for LtxVerifyError {}

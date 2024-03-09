use crate::LtxError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct LtxSchemeError {
  pub section: String,
  pub field: String,
  pub message: String,
}

impl LtxSchemeError {
  pub fn new<S, F, M>(section: S, field: F, message: M) -> LtxSchemeError
  where
    S: Into<String>,
    F: Into<String>,
    M: Into<String>,
  {
    LtxSchemeError {
      section: section.into(),
      field: field.into(),
      message: message.into(),
    }
  }

  pub fn new_ltx_error<S, F, M>(section: S, field: F, message: M) -> LtxError
  where
    S: Into<String>,
    F: Into<String>,
    M: Into<String>,
  {
    LtxError::Scheme(LtxSchemeError {
      section: section.into(),
      field: field.into(),
      message: message.into(),
    })
  }
}

impl Display for LtxSchemeError {
  fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
    write!(
      formatter,
      "Error in [{}] {}, reason: {}",
      self.section, self.field, self.message
    )
  }
}

impl Error for LtxSchemeError {}

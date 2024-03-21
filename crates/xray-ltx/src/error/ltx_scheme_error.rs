use crate::LtxError;
use serde::Serialize;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize)]
pub struct LtxSchemeError {
  pub section: String,
  pub field: String,
  pub message: String,
  pub at: Option<String>,
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
      at: None,
    }
  }

  pub fn new_at<S, F, M, A>(section: S, field: F, message: M, at: A) -> LtxSchemeError
  where
    S: Into<String>,
    F: Into<String>,
    M: Into<String>,
    A: Into<String>,
  {
    LtxSchemeError {
      section: section.into(),
      field: field.into(),
      message: message.into(),
      at: Some(at.into()),
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
      at: None,
    })
  }
}

impl Display for LtxSchemeError {
  fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
    if let Some(at) = &self.at {
      write!(
        formatter,
        "'{at}' [{}] {} : {}",
        self.section, self.field, self.message
      )
    } else {
      write!(
        formatter,
        "[{}] {} : {}",
        self.section, self.field, self.message
      )
    }
  }
}

impl Error for LtxSchemeError {}

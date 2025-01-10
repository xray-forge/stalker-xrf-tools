use serde::Serialize;
use std::io::Error as IoError;
use thiserror::Error as ThisError;

/// Error while working with LTX document.
#[derive(ThisError, Debug)]
pub enum LtxError {
  #[error("Ltx convert error: {message:?}")]
  Convert { message: String },
  #[error("Ltx format error: {message:?}")]
  Format { message: String },
  #[error("Ltx parse error: {line:?}:{col:?} {message:?}")]
  Parse {
    line: usize,
    col: usize,
    message: String,
  },
  #[error("Ltx read error: {message:?}")]
  Read { message: String },
  #[error("Ltx scheme error: '{at:?}' [{section:?}] {field:?} : {message:?}")]
  Scheme {
    section: String,
    field: String,
    message: String,
    at: Option<String>,
  },
  #[error("Ltx verify error: {message:?}")]
  Verify { message: String },
  #[error("Ltx IO error: {0}")]
  Io(#[from] IoError),
}

impl LtxError {
  pub fn new_read_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Read {
      message: message.into(),
    }
  }

  pub fn new_verify_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Verify {
      message: message.into(),
    }
  }

  pub fn new_convert_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Convert {
      message: message.into(),
    }
  }

  pub fn new_parse_error<T>(line: usize, col: usize, message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Parse {
      line,
      col,
      message: message.into(),
    }
  }

  pub fn new_scheme_error<S, F, M>(section: S, field: F, message: M) -> Self
  where
    S: Into<String>,
    F: Into<String>,
    M: Into<String>,
  {
    Self::Scheme {
      section: section.into(),
      field: field.into(),
      message: message.into(),
      at: None,
    }
  }

  pub fn new_scheme_error_at<S, F, M, A>(section: S, field: F, message: M, at: A) -> Self
  where
    S: Into<String>,
    F: Into<String>,
    M: Into<String>,
    A: Into<String>,
  {
    Self::Scheme {
      section: section.into(),
      field: field.into(),
      message: message.into(),
      at: Some(at.into()),
    }
  }
}

impl Serialize for LtxError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

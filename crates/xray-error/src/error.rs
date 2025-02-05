use serde::Serialize;
use std::io::ErrorKind;
use thiserror::Error as ThisError;
use xray_error_derive::ErrorConstructors;

/// Error while working with translation file
#[derive(ThisError, Debug, ErrorConstructors, Serialize)]
pub enum XRayError {
  #[constructor]
  #[error("Assertion error: {message}")]
  Assertion { message: String },
  #[constructor]
  #[error("Asset error: {message}")]
  Asset { message: String },
  #[constructor]
  #[error("Convert error: {message}")]
  Convert { message: String },
  #[constructor]
  #[error("Ltx format error: {message}")]
  Format { message: String },
  #[constructor]
  #[error("Verify error: {message}")]
  Verify { message: String },
  #[constructor]
  #[error("Not implemented error: {message}")]
  NotImplemented { message: String },
  #[constructor]
  #[error("Read error: {message}")]
  Read { message: String },
  #[constructor]
  #[error("Unexpected error: {message}")]
  Unexpected { message: String },
  #[constructor]
  #[error("Not found error: {message}")]
  NotFound { message: String },
  #[constructor]
  #[error("Invalid error: {message}")]
  Invalid { message: String },
  #[constructor]
  #[error("Parsing error: {message}")]
  Parsing { message: String },
  #[constructor]
  #[error("Encoding error: {message}")]
  Encoding { message: String },
  #[constructor]
  #[error("Missing terminator error: {message}")]
  NoTerminator { message: String },
  #[constructor]
  #[error("Unknown language: {message}")]
  UnknownLanguage { message: String },
  #[constructor]
  #[error("Invalid source: {message}")]
  InvalidSource { message: String },
  #[constructor]
  #[error("Texture processing error: {message}")]
  TextureProcessing { message: String },
  #[error("Chunk is not ended, {remaining} bytes remain: {message}")]
  ChunkNotEnded { message: String, remaining: u64 },
  #[error("Ltx parse error: {line}:{col} {message}")]
  LtxParse {
    line: usize,
    col: usize,
    message: String,
  },
  #[error("Ltx scheme error: '{at:?}' [{section}] {field} : {message}")]
  LtxScheme {
    section: String,
    field: String,
    message: String,
    at: Option<String>,
  },
  #[constructor]
  #[error("Generic error: {message}")]
  Generic { message: String },
  #[constructor]
  #[error("Serde error: {message}")]
  Serde { message: String },
  #[error("IO error: {message}")]
  Io {
    message: String,
    #[serde(skip_serializing)]
    kind: ErrorKind,
  },
}

impl XRayError {
  pub fn new_chunk_not_ended_error<T>(message: T, remaining: u64) -> Self
  where
    T: Into<String>,
  {
    Self::ChunkNotEnded {
      message: message.into(),
      remaining,
    }
  }

  pub fn new_ltx_parse_error<T>(line: usize, col: usize, message: T) -> Self
  where
    T: Into<String>,
  {
    Self::LtxParse {
      line,
      col,
      message: message.into(),
    }
  }

  pub fn new_ltx_scheme_error<S, F, M>(section: S, field: F, message: M) -> Self
  where
    S: Into<String>,
    F: Into<String>,
    M: Into<String>,
  {
    Self::LtxScheme {
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
    Self::LtxScheme {
      section: section.into(),
      field: field.into(),
      message: message.into(),
      at: Some(at.into()),
    }
  }

  pub fn new_io_error<T>(message: T, kind: ErrorKind) -> Self
  where
    T: Into<String>,
  {
    Self::Io {
      message: message.into(),
      kind,
    }
  }
}

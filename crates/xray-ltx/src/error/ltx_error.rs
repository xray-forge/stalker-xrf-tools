use crate::error::ltx_convert_error::LtxConvertError;
use crate::error::ltx_parse_error::LtxParseError;
use crate::error::ltx_read_error::LtxReadError;
use crate::error::ltx_scheme_error::LtxSchemeError;
use crate::{LtxFormatError, LtxVerifyError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

/// Error while working with LTX document.
#[derive(Debug)]
pub enum LtxError {
  Convert(LtxConvertError),
  Format(LtxFormatError),
  Io(IoError),
  Parse(LtxParseError),
  Read(LtxReadError),
  Scheme(LtxSchemeError),
  Verify(LtxVerifyError),
}

impl Display for LtxError {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    match *self {
      Self::Convert(ref error) => error.fmt(formatter),
      Self::Format(ref error) => error.fmt(formatter),
      Self::Io(ref error) => error.fmt(formatter),
      Self::Parse(ref error) => error.fmt(formatter),
      Self::Read(ref error) => error.fmt(formatter),
      Self::Scheme(ref error) => error.fmt(formatter),
      Self::Verify(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for LtxError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::Convert(ref error) => error.source(),
      Self::Format(ref error) => error.source(),
      Self::Io(ref error) => error.source(),
      Self::Parse(ref error) => error.source(),
      Self::Read(ref error) => error.source(),
      Self::Scheme(ref error) => error.source(),
      Self::Verify(ref error) => error.source(),
    }
  }
}

impl From<IoError> for LtxError {
  fn from(error: IoError) -> Self {
    Self::Io(error)
  }
}

impl From<LtxReadError> for LtxError {
  fn from(error: LtxReadError) -> Self {
    Self::Read(error)
  }
}

impl From<LtxParseError> for LtxError {
  fn from(error: LtxParseError) -> Self {
    Self::Parse(error)
  }
}

impl From<LtxFormatError> for LtxError {
  fn from(error: LtxFormatError) -> Self {
    Self::Format(error)
  }
}

impl From<LtxVerifyError> for LtxError {
  fn from(error: LtxVerifyError) -> Self {
    Self::Verify(error)
  }
}

impl TryInto<LtxParseError> for LtxError {
  type Error = LtxError;

  fn try_into(self) -> Result<LtxParseError, Self::Error> {
    match self {
      Self::Parse(error) => Ok(error),
      error => Err(error),
    }
  }
}

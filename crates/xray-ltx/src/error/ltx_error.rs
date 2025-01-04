use crate::error::ltx_convert_error::LtxConvertError;
use crate::error::ltx_parse_error::LtxParseError;
use crate::error::ltx_read_error::LtxReadError;
use crate::error::ltx_scheme_error::LtxSchemeError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

/// Error while working with LTX document.
#[derive(Debug)]
pub enum LtxError {
  Io(io::Error),
  Parse(LtxParseError),
  Convert(LtxConvertError),
  Read(LtxReadError),
  Scheme(LtxSchemeError),
}

impl Display for LtxError {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    match *self {
      LtxError::Io(ref error) => error.fmt(formatter),
      LtxError::Parse(ref error) => error.fmt(formatter),
      LtxError::Convert(ref error) => error.fmt(formatter),
      LtxError::Read(ref error) => error.fmt(formatter),
      LtxError::Scheme(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for LtxError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      LtxError::Io(ref error) => error.source(),
      LtxError::Parse(ref error) => error.source(),
      LtxError::Convert(ref error) => error.source(),
      LtxError::Read(ref error) => error.source(),
      LtxError::Scheme(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for LtxError {
  fn from(error: io::Error) -> Self {
    LtxError::Io(error)
  }
}

impl From<LtxReadError> for LtxError {
  fn from(error: LtxReadError) -> Self {
    LtxError::Read(error)
  }
}

impl From<LtxParseError> for LtxError {
  fn from(error: LtxParseError) -> Self {
    LtxError::Parse(error)
  }
}

impl TryInto<LtxParseError> for LtxError {
  type Error = LtxError;

  fn try_into(self) -> Result<LtxParseError, Self::Error> {
    match self {
      LtxError::Parse(error) => Ok(error),
      error => Err(error),
    }
  }
}

use crate::error::ltx_convert_error::LtxConvertError;
use crate::error::ltx_parse_error::LtxParseError;
use crate::error::ltx_read_error::LtxReadError;
use crate::error::ltx_scheme_error::LtxSchemeError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
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
  fn fmt(&self, formatter: &mut Formatter) -> Result {
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
  fn from(err: io::Error) -> Self {
    LtxError::Io(err)
  }
}

impl From<LtxReadError> for LtxError {
  fn from(item: LtxReadError) -> Self {
    LtxError::Read(item)
  }
}

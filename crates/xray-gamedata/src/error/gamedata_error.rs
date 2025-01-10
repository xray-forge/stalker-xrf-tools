use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;
use xray_db::DatabaseError;
use xray_ltx::LtxError;

#[derive(Debug)]
pub enum GamedataError {
  Database(DatabaseError),
  Io(io::Error),
  Ltx(LtxError),
  Generic(Box<dyn Error>),
}

impl Display for GamedataError {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    match *self {
      Self::Database(ref error) => error.fmt(formatter),
      Self::Io(ref error) => error.fmt(formatter),
      Self::Ltx(ref error) => error.fmt(formatter),
      Self::Generic(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for GamedataError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::Database(ref error) => error.source(),
      Self::Io(ref error) => error.source(),
      Self::Ltx(ref error) => error.source(),
      Self::Generic(ref error) => error.source(),
    }
  }
}

impl From<io::Error> for GamedataError {
  fn from(error: io::Error) -> Self {
    Self::Io(error)
  }
}

impl From<LtxError> for GamedataError {
  fn from(error: LtxError) -> Self {
    Self::Ltx(error)
  }
}

impl From<Box<dyn Error>> for GamedataError {
  fn from(error: Box<dyn Error>) -> Self {
    Self::Generic(error)
  }
}

impl From<DatabaseError> for GamedataError {
  fn from(error: DatabaseError) -> Self {
    Self::Database(error)
  }
}

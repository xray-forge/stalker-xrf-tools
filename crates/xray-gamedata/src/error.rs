use std::error::Error;
use std::io;
use std::path::StripPrefixError;
use thiserror::Error as ThisError;
use xray_db::DatabaseError;
use xray_ltx::LtxError;

#[derive(ThisError, Debug)]
pub enum GamedataError {
  #[error("Gamedata check error: \"{message:?}\"")]
  Check { message: String },
  #[error("Gamedata asset error: \"{message:?}\"")]
  Asset { message: String },
  #[error("Database error: \"{0}\"")]
  Database(#[from] DatabaseError),
  #[error("Database IO error: \"{0}\"")]
  Io(#[from] io::Error),
  #[error("Database LTX error: \"{0}\"")]
  Ltx(#[from] LtxError),
  #[error("Database generic error: \"{0}\"")]
  Generic(#[from] Box<dyn Error>),
}

impl GamedataError {
  pub fn new_asset_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Check {
      message: message.into(),
    }
  }

  pub fn new_check_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Check {
      message: message.into(),
    }
  }
}

impl From<StripPrefixError> for GamedataError {
  fn from(_: StripPrefixError) -> Self {
    Self::new_check_error("Failed to strip prefix from string")
  }
}

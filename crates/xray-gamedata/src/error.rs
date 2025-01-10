use std::error::Error;
use std::io;
use thiserror::Error as ThisError;
use xray_db::DatabaseError;
use xray_ltx::LtxError;

#[derive(ThisError, Debug)]
pub enum GamedataError {
  #[error("Database error: {0}")]
  Database(#[from] DatabaseError),
  #[error("Database IO error: {0}")]
  Io(#[from] io::Error),
  #[error("Database LTX error: {0}")]
  Ltx(#[from] LtxError),
  #[error("Database generic error: {0}")]
  Generic(#[from] Box<dyn Error>),
}

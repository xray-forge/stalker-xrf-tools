use crate::error::database_error::DatabaseError;
use crate::types::DatabaseResult;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use xray_ltx::Ltx;

/// Create file for exporting by provided path.
pub fn create_export_file(path: &Path) -> DatabaseResult<File> {
  match OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path)
  {
    Ok(file) => Ok(file),
    Err(error) => Err(
      io::Error::new(
        error.kind(),
        format!("Failed to create file for spawn output: {:?}", path),
      )
      .into(),
    ),
  }
}

/// Try opening ltx file.
/// Map any ltx reading operation errors as IO invalid input.
pub fn open_ltx_config(path: &Path) -> DatabaseResult<Ltx> {
  Ltx::read_from_path(path).map_err(DatabaseError::from)
}

/// Try opening binary data storing file.
pub fn open_binary_file(path: &Path) -> DatabaseResult<File> {
  File::open(path).map_err(DatabaseError::from)
}

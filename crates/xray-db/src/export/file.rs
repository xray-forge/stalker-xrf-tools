use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use xray_ltx::Ltx;

/// Create file for exporting by provided path.
pub fn create_export_file(path: &Path) -> io::Result<File> {
  match OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path)
  {
    Ok(file) => Ok(file),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to create file for spawn output {:?}", path),
    )),
  }
}

/// Try opening ini file.
/// Map any ini reading operation errors as IO invalid input.
pub fn open_ini_config(path: &Path) -> io::Result<Ltx> {
  match Ltx::read_from_file(path) {
    Ok(ini) => Ok(ini),
    Err(error) => Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      error.to_string(),
    )),
  }
}

/// Try opening binary data storing file.
pub fn open_binary_file(path: &Path) -> io::Result<File> {
  File::open(path)
}

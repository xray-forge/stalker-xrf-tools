use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;

pub fn create_export_file(path: &PathBuf) -> io::Result<File> {
  match OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path.clone())
  {
    Ok(file) => Ok(file),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to create file for spawn output {:?}", path),
    )),
  }
}

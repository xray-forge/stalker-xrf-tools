use std::fs::{File, OpenOptions};
use std::io::{Error as IoError, Read, Result as IoResult};
use std::path::Path;

/// Read whole file as string.
pub fn read_file_as_string(file: &mut File) -> IoResult<String> {
  let mut value: String = String::new();

  file.read_to_string(&mut value)?;

  Ok(value)
}

/// Create and open file by path, overwrite existing one.
pub fn overwrite_file(path: &Path) -> IoResult<File> {
  std::fs::create_dir_all(path.parent().expect("Parent directory"))?;

  match OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .read(true)
    .open(path)
  {
    Ok(file) => Ok(file),
    Err(error) => Err(IoError::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

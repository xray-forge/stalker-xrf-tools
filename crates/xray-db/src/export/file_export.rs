use encoding_rs::WINDOWS_1251;
use ini::{Ini, WriteOption};
use std::borrow::Cow;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

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

/// Export ini file content to provided file.
pub fn export_ini_to_file(ini: &Ini, file: &mut File) -> io::Result<()> {
  ini.write_to_opt(
    file,
    WriteOption {
      kv_separator: " = ",
      ..Default::default()
    },
  )
}

/// Export ini file content to provided file.
pub fn export_vector_to_string<T: Display>(vector: &Vec<T>) -> String {
  vector
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
    .join(",")
}

/// Export ini file content to provided file.
pub fn export_bytes_to_windows_1251_string(buffer: &Vec<u8>) -> String {
  let (transformed, _, had_errors) = WINDOWS_1251.decode(buffer);

  if had_errors {
    panic!("Unexpected errors when decoding windows-1251 string data.");
  }

  // Try with windows 1251 conversion:
  match transformed {
    Cow::Borrowed(value) => String::from(value),
    Cow::Owned(value) => value,
  }
}

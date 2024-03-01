use ini::{EscapePolicy, Ini, ParseOption, WriteOption};
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
      escape_policy: EscapePolicy::Nothing,
      ..Default::default()
    },
  )
}

/// Try opening ini file.
/// Map any ini reading operation errors as IO invalid input.
pub fn open_ini_config(path: &Path) -> io::Result<Ini> {
  match Ini::load_from_file_opt(
    path,
    ParseOption {
      enabled_escape: false,
      enabled_quote: false,
    },
  ) {
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

use ini::Ini;
use std::io;
use std::path::Path;

/// Try opening ini file.
/// Map any ini reading operation errors as IO invalid input.
pub fn open_ini_config(path: &Path) -> io::Result<Ini> {
  match Ini::load_from_file(path) {
    Ok(ini) => Ok(ini),
    Err(error) => Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      error.to_string(),
    )),
  }
}

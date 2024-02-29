use ini::Ini;
use std::fs::File;
use std::io;
use std::path::Path;
use std::str::FromStr;

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

/// Try opening binary data storing file.
pub fn open_binary_file(path: &Path) -> io::Result<File> {
  File::open(path)
}

/// Export ini file content to provided file.
#[allow(dead_code)]
pub fn import_vector_from_string<T: FromStr>(value: &String) -> io::Result<Vec<T>> {
  let mut vector: Vec<T> = Vec::new();

  for it in value.split(",") {
    vector.push(match it.trim().parse::<T>() {
      Ok(v) => v,
      _ => {
        return Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          String::from("Failed to parse vector from string."),
        ))
      }
    });
  }

  Ok(vector)
}

/// Export ini file content to provided file.
pub fn import_sized_vector_from_string<T: FromStr>(
  size: usize,
  value: &String,
) -> io::Result<Vec<T>> {
  let mut vector: Vec<T> = Vec::new();

  for (index, it) in value.split(",").enumerate() {
    vector.push(match it.trim().parse::<T>() {
      Ok(v) => v,
      _ => {
        return Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          String::from("Failed to parse sized vector from string"),
        ))
      }
    });

    if index >= size {
      return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        String::from(
          "Failed to parse sized vector from string, it has more elements than required",
        ),
      ));
    }
  }

  if vector.len() != size {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      String::from("Failed to parse sized vector from string, it has less elements than required"),
    ));
  }

  Ok(vector)
}

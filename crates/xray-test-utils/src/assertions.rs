use std::fs::File;
use std::io::{BufReader, Read, Result as IoResult};
use std::path::Path;

/// Assert file content is equal.
pub fn files_are_equal_by_path<P: AsRef<Path>>(first_path: P, second_path: P) -> IoResult<bool> {
  files_are_equal(
    File::open(first_path.as_ref())?,
    File::open(second_path.as_ref())?,
  )
}

/// Assert file content is equal.
pub fn files_are_equal(first: File, second: File) -> IoResult<bool> {
  let mut first_reader: BufReader<File> = BufReader::new(first);
  let mut second_reader: BufReader<File> = BufReader::new(second);

  if first_reader.get_ref().metadata()?.len() != second_reader.get_ref().metadata()?.len() {
    return Ok(false);
  }

  let mut first_buffer: [u8; 4096] = [0; 4096];
  let mut second_buffer: [u8; 4096] = [0; 4096];

  loop {
    match (
      first_reader.read(&mut first_buffer)?,
      second_reader.read(&mut second_buffer)?,
    ) {
      (0, 0) => return Ok(true),
      (first_read, second_read)
        if first_read == second_read
          && first_buffer[..first_read] == second_buffer[..second_read] =>
      {
        continue
      }
      _ => return Ok(false),
    }
  }
}

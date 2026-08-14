use std::fs::{File, OpenOptions};
use std::path::Path;

use xrf_error::{XrfError, XrfResult};

/// Create file for exporting by provided path.
#[inline]
pub fn open_export_file<T: AsRef<Path>>(path: T) -> XrfResult<File> {
  OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path)
    .map_err(|error| XrfError::new_io_error(format!("Failed to create file for exporting: {}", error), error.kind()))
}

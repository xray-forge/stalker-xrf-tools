use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use xray_error::{XRayError, XRayResult};

/// Create file for exporting by provided path.
pub fn create_export_file(path: &Path) -> XRayResult<File> {
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
        format!("Failed to create file for spawn output: {}", path.display()),
      )
      .into(),
    ),
  }
}

// todo:
// todo:
// todo:
/// Try opening binary data storing file.
pub fn open_binary_file(path: &Path) -> XRayResult<File> {
  File::open(path).map_err(XRayError::from)
}

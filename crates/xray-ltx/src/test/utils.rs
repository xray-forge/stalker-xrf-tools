use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

/// Get absolute path to provided test resource.
pub fn get_absolute_test_resource_path(resource_path: &Path) -> PathBuf {
  let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

  path.push("resources");
  path.push("tests");
  path.push(resource_path);

  path
}

/// Get relative path to sample resource.
pub fn get_relative_test_file_path(file: &str, resource: &str) -> PathBuf {
  let mut path: PathBuf = PathBuf::new();

  path.push(Path::new(file).file_stem().unwrap());
  path.push(resource);

  path
}

/// Get Absolute path to sample resource.
pub fn get_absolute_test_file_path(file: &str, resource: &str) -> PathBuf {
  get_absolute_test_resource_path(&get_relative_test_file_path(file, resource))
}

/// Open file from test resources.
pub fn get_absolute_test_resource_as_file(file: &str, resource: &str) -> io::Result<File> {
  let path: PathBuf = get_absolute_test_file_path(file, resource);

  match File::open(&path) {
    Ok(file) => Ok(file),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

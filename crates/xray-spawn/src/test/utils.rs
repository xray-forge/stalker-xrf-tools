use fileslice::FileSlice;
use std::fs::File;
use std::io;
use std::path::PathBuf;

/// Get absolute resources directory.
pub fn get_resources_path() -> PathBuf {
  let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

  path.push("resources");

  path
}

/// Get absolute path to provided test resource.
pub fn get_test_resource_path(resource_path: String) -> PathBuf {
  let mut path: PathBuf = get_resources_path();

  path.push("tests");
  path.push(resource_path);

  path
}

/// Open file from test resources as slice.
pub fn open_test_resource_as_slice(resource_path: String) -> io::Result<FileSlice> {
  let path: PathBuf = get_test_resource_path(resource_path);

  match File::open(path.clone()) {
    Ok(file) => Ok(FileSlice::new(file)),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

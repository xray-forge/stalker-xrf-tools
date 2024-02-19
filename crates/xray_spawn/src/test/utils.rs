use fileslice::FileSlice;
use std::fs::File;
use std::io;
use std::path::PathBuf;

/// todo;
pub fn get_resources_path() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// todo;
pub fn get_test_resource_path(resource_path: String) -> PathBuf {
  let mut path: PathBuf = get_resources_path();

  path.push("resources\\tests");
  path.push(resource_path);

  path
}

/// todo;
pub fn open_test_resource_as_slice(resource_path: String) -> io::Result<FileSlice> {
  let path: PathBuf = get_test_resource_path(resource_path);

  Ok(FileSlice::new(File::open(path.clone()).unwrap()))
}

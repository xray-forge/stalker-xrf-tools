use fileslice::FileSlice;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

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

/// Get relative path to chunk resource.
pub fn get_test_chunk_file_sub_dir(file: &str, resource: String) -> String {
  let mut path: PathBuf = PathBuf::new();

  path.push("chunks");
  path.push(Path::new(file).file_stem().unwrap());
  path.push(resource);

  path.into_os_string().into_string().unwrap()
}

/// Get relative path to chunk resource.
pub fn get_test_chunk_sub_dir(resource: String) -> String {
  let mut path: PathBuf = PathBuf::new();

  path.push("chunks");
  path.push(resource);

  path.into_os_string().into_string().unwrap()
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

/// Open file from test resources.
pub fn open_test_resource_as_file(resource_path: String) -> io::Result<File> {
  let path: PathBuf = get_test_resource_path(resource_path);

  match File::open(path.clone()) {
    Ok(file) => Ok(file),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

/// Create and open file from test resources, overwrite existing one.
pub fn overwrite_test_resource_as_file(resource_path: String) -> io::Result<File> {
  let path: PathBuf = get_test_resource_path(resource_path);

  std::fs::create_dir_all(path.parent().expect("Parent directory"))?;

  match OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path.clone())
  {
    Ok(file) => Ok(file),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

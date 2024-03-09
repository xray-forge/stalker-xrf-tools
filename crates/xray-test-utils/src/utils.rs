use fileslice::FileSlice;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

/// Get absolute path to provided test resource.
pub fn get_absolute_test_resource_path(resource_path: &str) -> PathBuf {
  let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

  path.push("resources");
  path.push("tests");
  path.push(resource_path);

  path
}

/// Get Absolute path to sample resource.
pub fn get_absolute_test_file_path(file: &str, resource: &str) -> PathBuf {
  get_absolute_test_resource_path(
    &get_relative_test_file_path(file, resource)
      .into_os_string()
      .into_string()
      .unwrap(),
  )
}

/// Get relative path to sample resource.
pub fn get_relative_test_file_path(file: &str, resource: &str) -> PathBuf {
  let mut path: PathBuf = PathBuf::new();

  path.push(Path::new(file).file_stem().unwrap());
  path.push(resource);

  path
}

/// Get relative path to sample resource.
pub fn get_relative_test_sample_file_path(file: &str, resource: &str) -> String {
  let mut path: PathBuf = PathBuf::new();

  path.push(Path::new(file).file_stem().unwrap());
  path.push(resource);

  path.into_os_string().into_string().unwrap()
}

/// Get Absolute path to sample resource.
pub fn get_absolute_test_sample_file_path(file: &str, resource: &str) -> PathBuf {
  get_absolute_test_resource_path(&get_relative_test_sample_file_path(file, resource))
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

/// Get relative path to sample resource of current test file.
pub fn get_relative_test_sample_file_directory(file: &str) -> String {
  let mut path: PathBuf = PathBuf::new();

  path.push(Path::new(file).file_stem().unwrap());

  path.into_os_string().into_string().unwrap()
}

/// Get relative path to sample resource.
pub fn get_relative_test_sample_sub_dir(resource: &str) -> String {
  let mut path: PathBuf = PathBuf::new();

  path.push(resource);

  path.into_os_string().into_string().unwrap()
}

/// Open file from test resources as slice.
pub fn open_test_resource_as_slice(resource_path: &str) -> io::Result<FileSlice> {
  let path: PathBuf = get_absolute_test_resource_path(resource_path);

  match File::open(path.clone()) {
    Ok(file) => Ok(FileSlice::new(file)),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

/// Open file from test resources.
pub fn open_test_resource_as_file(resource_path: &str) -> io::Result<File> {
  let path: PathBuf = get_absolute_test_resource_path(resource_path);

  match File::open(path.clone()) {
    Ok(file) => Ok(file),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

/// Create and open file from test resources, overwrite existing one.
pub fn overwrite_test_relative_resource_as_file(resource_path: &str) -> io::Result<File> {
  let path: PathBuf = get_absolute_test_resource_path(resource_path);

  std::fs::create_dir_all(path.parent().expect("Parent directory"))?;

  match OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .read(true)
    .open(path.clone())
  {
    Ok(file) => Ok(file),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

/// Create and open file by path, overwrite existing one.
pub fn overwrite_file(path: &Path) -> io::Result<File> {
  std::fs::create_dir_all(path.parent().expect("Parent directory"))?;

  match OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .read(true)
    .open(path)
  {
    Ok(file) => Ok(file),
    Err(error) => Err(io::Error::new(
      error.kind(),
      format!("Failed to open test asset {:?}", path),
    )),
  }
}

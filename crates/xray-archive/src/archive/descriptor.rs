use crate::archive::file_descriptor::ArchiveFileDescriptor;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize)]
pub struct ArchiveDescriptor {
  #[serde(rename = "files")]
  pub files: HashMap<String, ArchiveFileDescriptor>,
  #[serde(rename = "outputRootPath")]
  pub output_root_path: PathBuf,
  #[serde(rename = "path")]
  pub path: PathBuf,
}

impl ArchiveDescriptor {
  pub fn is_valid_db_path(path: &Path) -> bool {
    match path.extension() {
      None => false,
      Some(ext) => {
        let ext: &str = ext.to_str().unwrap();

        ext.starts_with("db") || ext.starts_with("xdb")
      }
    }
  }
}

impl ArchiveDescriptor {
  pub fn get_real_size(&self) -> u64 {
    let mut total: u64 = 0;

    for file in self.files.values() {
      total += file.size_real as u64;
    }

    total
  }

  pub fn get_compressed_size(&self) -> u64 {
    let mut total: u64 = 0;

    for file in self.files.values() {
      total += file.size_compressed as u64;
    }

    total
  }
}

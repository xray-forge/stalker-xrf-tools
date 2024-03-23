use crate::archive::file_descriptor::ArchiveFileDescriptor;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct ArchiveDescriptor {
  pub path: PathBuf,
  pub output_root_path: PathBuf,
  pub files: HashMap<String, ArchiveFileDescriptor>,
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

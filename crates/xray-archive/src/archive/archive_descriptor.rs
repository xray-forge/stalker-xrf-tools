use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveDescriptor {
  pub files: HashMap<String, ArchiveFileDescriptor>,
  pub output_root_path: PathBuf,
  pub path: PathBuf,
}

impl ArchiveDescriptor {
  pub fn is_valid_db_path<P: AsRef<Path>>(path: &P) -> bool {
    match path.as_ref().extension() {
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

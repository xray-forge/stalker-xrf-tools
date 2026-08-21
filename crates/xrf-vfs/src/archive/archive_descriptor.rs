use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;

#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveDescriptor {
  pub created_at: Option<u64>,
  pub modified_at: Option<u64>,
  pub files: HashMap<String, ArchiveFileDescriptor>,
  pub output_root_path: PathBuf,
  pub path: PathBuf,
}

impl ArchiveDescriptor {
  /// Whether a path names an archive volume by extension, matching `.db*` and `.xdb*` without case.
  ///
  /// Case-insensitive to agree with the mount planner's volume detection; a non-UTF-8 extension is not a volume rather
  /// than a panic.
  pub fn is_valid_db_path(path: impl AsRef<Path>) -> bool {
    path
      .as_ref()
      .extension()
      .and_then(|extension| extension.to_str())
      .is_some_and(|extension| {
        let extension: String = extension.to_ascii_lowercase();

        extension.starts_with("db") || extension.starts_with("xdb")
      })
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

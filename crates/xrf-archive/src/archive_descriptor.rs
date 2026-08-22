use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::archive_file_descriptor::ArchiveFileDescriptor;

/// One volume's parsed header: its entry table and the gamedata-relative root its `[header] entry_point` declares.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveDescriptor {
  /// Volume file creation time in Unix milliseconds, when the filesystem reports one.
  pub created_at: Option<u64>,
  /// Volume file modification time in Unix milliseconds, when the filesystem reports one.
  pub modified_at: Option<u64>,
  /// Entries keyed by their authored name, exactly as the name table records them.
  pub files: HashMap<String, ArchiveFileDescriptor>,
  /// Root the volume unpacks under, from `[header] entry_point` with its alias stripped.
  pub output_root_path: PathBuf,
  /// The volume file this descriptor was read from.
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

  /// Bytes this volume's entries occupy once unpacked.
  pub fn get_real_size(&self) -> u64 {
    self.files.values().map(|file| u64::from(file.size_real)).sum()
  }

  /// Bytes this volume's entries occupy as stored.
  pub fn get_compressed_size(&self) -> u64 {
    self.files.values().map(|file| u64::from(file.size_compressed)).sum()
  }
}

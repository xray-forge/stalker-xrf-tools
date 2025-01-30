use crate::archive::archive_descriptor::ArchiveDescriptor;
use crate::archive::archive_file_descriptor::ArchiveFileReplicationDescriptor;
use crate::archive::reader::ArchiveReader;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use xray_error::{XRayError, XRayResult};

// todo: Add reading from fsgame.ltx file.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveProject {
  pub archives: Vec<ArchiveDescriptor>,
  pub files: HashMap<String, ArchiveFileReplicationDescriptor>,
}

impl ArchiveProject {
  pub fn new(path: &Path) -> XRayResult<Self> {
    let mut archives: Vec<ArchiveDescriptor> = Vec::new();
    let mut files: HashMap<String, ArchiveFileReplicationDescriptor> = HashMap::new();

    if path.is_file() {
      log::info!("Reading archive file: {}", path.display());

      archives.push(ArchiveReader::from_path_utf8(path)?.read_archive()?);
    } else {
      log::info!("Reading archive folder: {}", path.display());

      for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| match entry {
          Ok(entry) => Some(entry),
          Err(_) => None,
        })
      {
        let path: &Path = entry.path();

        if ArchiveDescriptor::is_valid_db_path(path) {
          log::info!("Reading archive file: {}", path.display());

          archives.push(ArchiveReader::from_path_utf8(path)?.read_archive()?);
        }
      }
    }

    if archives.is_empty() {
      return Err(XRayError::new_read_error(format!(
        "Unable to read archives at location {}",
        path.display()
      )));
    }

    Self::sort_archives(&mut archives);

    for archive in &archives {
      for (name, descriptor) in &archive.files {
        files.insert(
          name.clone(),
          ArchiveFileReplicationDescriptor::from_descriptor(
            descriptor,
            &archive.path,
            &archive.output_root_path,
          ),
        );
      }
    }

    Ok(Self { archives, files })
  }
}

impl ArchiveProject {
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

  /// Sort archives list to maintain overriding of files in a correct way.
  /// Patches are exceptional case and should override all the files.
  fn sort_archives(archives: &mut [ArchiveDescriptor]) {
    archives.sort_by(|first, second| {
      let first: &str = first.path.to_str().unwrap();
      let second: &str = second.path.to_str().unwrap();

      if first.contains("patches") {
        if second.contains("patches") {
          first.cmp(second)
        } else {
          Ordering::Greater
        }
      } else {
        if second.contains("patches") {
          Ordering::Less
        } else {
          first.cmp(second)
        }
      }
    });
  }
}

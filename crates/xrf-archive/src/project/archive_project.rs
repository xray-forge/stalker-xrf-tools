use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;
use xrf_error::{XRayError, XRayResult};

use crate::archive::archive_descriptor::ArchiveDescriptor;
use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
use crate::archive::reader::ArchiveReader;
use crate::project::archive_project_read_policy::ArchiveProjectReadPolicy;

// todo: Add reading from fsgame.ltx file.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveProject {
  pub archives: Vec<ArchiveDescriptor>,
  pub files: HashMap<String, ArchiveFileDescriptor>,
  pub read_policy: ArchiveProjectReadPolicy,
  pub root: PathBuf,
  pub size_real: u64,
}

impl ArchiveProject {
  pub fn new<P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    let mut archives: Vec<ArchiveDescriptor> = Vec::new();
    let mut files: HashMap<String, ArchiveFileDescriptor> = HashMap::new();

    if path.as_ref().is_file() {
      log::info!("Reading archive file: {}", path.as_ref().display());

      archives.push(ArchiveReader::from_path_windows1251(path)?.read_archive()?);
    } else {
      log::info!("Reading archive folder: {}", path.as_ref().display());

      for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let path: &Path = entry.path();

        if ArchiveDescriptor::is_valid_db_path(&path) {
          log::info!("Reading archive file: {}", path.display());

          archives.push(ArchiveReader::from_path_windows1251(&path)?.read_archive()?);
        }
      }
    }

    if archives.is_empty() {
      return Err(XRayError::new_read_error(format!(
        "Unable to read archives at location {}",
        path.as_ref().display()
      )));
    }

    Self::sort_archives(&mut archives);

    for archive in &archives {
      for (name, descriptor) in &archive.files {
        files.insert(name.clone(), descriptor.clone());
      }
    }

    let root: PathBuf = Self::root_from_archives(&archives);
    let size_real: u64 = files.values().map(|file| u64::from(file.size_real)).sum();

    Ok(Self {
      archives,
      files,
      read_policy: ArchiveProjectReadPolicy::default(),
      root,
      size_real,
    })
  }
}

impl ArchiveProject {
  pub fn get_real_size(&self) -> u64 {
    self.size_real
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
        // Handle second path:
        if second.contains("patches") {
          Ordering::Less
        } else {
          first.cmp(second)
        }
      }
    });
  }

  fn root_from_archives(archives: &[ArchiveDescriptor]) -> PathBuf {
    let Some(first) = archives.first() else {
      return PathBuf::new();
    };
    let mut common: Vec<OsString> = first
      .path
      .parent()
      .unwrap_or_else(|| Path::new(""))
      .components()
      .map(|component| component.as_os_str().to_owned())
      .collect();

    for archive in &archives[1..] {
      let components: Vec<OsString> = archive
        .path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .components()
        .map(|component| component.as_os_str().to_owned())
        .collect();
      let common_length: usize = common
        .iter()
        .zip(&components)
        .take_while(|(left, right)| left == right)
        .count();

      common.truncate(common_length);
    }

    common.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use std::path::Path;

  use crate::archive::archive_descriptor::ArchiveDescriptor;

  use super::ArchiveProject;

  #[test]
  fn project_root_is_the_common_parent_of_all_archives() {
    let archives = [
      archive("/game/database/configs.db0"),
      archive("/game/database/patches/patch.db"),
    ];

    assert_eq!(
      ArchiveProject::root_from_archives(&archives),
      Path::new("/game/database")
    );
  }

  fn archive(path: &str) -> ArchiveDescriptor {
    ArchiveDescriptor {
      created_at: None,
      files: HashMap::new(),
      modified_at: None,
      output_root_path: Path::new("gamedata").into(),
      path: Path::new(path).into(),
    }
  }
}

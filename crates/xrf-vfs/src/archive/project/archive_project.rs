use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;
use xrf_error::{XrfError, XrfResult};

use crate::archive::archive_descriptor::ArchiveDescriptor;
use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
use crate::archive::project::archive_project_read_policy::ArchiveProjectReadPolicy;
use crate::archive::reader::ArchiveReader;

// todo: Add reading from fsgame.ltx file.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveProject {
  pub archives: Vec<ArchiveDescriptor>,
  pub files: HashMap<String, ArchiveFileDescriptor>,
  pub read_policy: ArchiveProjectReadPolicy,
  pub root: PathBuf,
  pub size_real: u64,
}

impl ArchiveProject {
  /// Reads one archive file or all archive volumes recursively under a directory.
  ///
  /// # Errors
  ///
  /// Returns an error when no archive volume is found or a volume cannot be read.
  pub fn new<P: AsRef<Path>>(path: &P) -> XrfResult<Self> {
    Self::read_to_depth(path, usize::MAX)
  }

  /// Reads one archive file or archive volumes directly under a directory.
  ///
  /// Use this for nonrecursive `fsgame.ltx` archive aliases; recursive discovery would include subdirectories planned as
  /// separate mounts.
  ///
  /// # Errors
  ///
  /// Returns an error when no archive volume is found or a volume cannot be read.
  pub fn new_shallow<P: AsRef<Path>>(path: &P) -> XrfResult<Self> {
    Self::read_to_depth(path, 1)
  }

  fn read_to_depth<P: AsRef<Path>>(path: &P, depth: usize) -> XrfResult<Self> {
    let mut archives: Vec<ArchiveDescriptor> = Vec::new();
    let mut files: HashMap<String, ArchiveFileDescriptor> = HashMap::new();

    if path.as_ref().is_file() {
      log::info!("Reading archive file: {}", path.as_ref().display());

      archives.push(ArchiveReader::from_path_windows1251(path)?.read_archive()?);
    } else {
      log::info!("Reading archive directory: {}", path.as_ref().display());

      for entry in WalkDir::new(path).max_depth(depth).into_iter().filter_map(Result::ok) {
        let path: &Path = entry.path();

        if ArchiveDescriptor::is_valid_db_path(&path) {
          log::info!("Reading archive file: {}", path.display());

          archives.push(ArchiveReader::from_path_windows1251(&path)?.read_archive()?);
        }
      }
    }

    if archives.is_empty() {
      return Err(XrfError::new_read_error(format!(
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
  /// Patches are exceptional case and should override all the files, so they sort last and win the name-table merge.
  fn sort_archives(archives: &mut [ArchiveDescriptor]) {
    archives.sort_by(|first, second| {
      Self::is_patch_volume(&first.path)
        .cmp(&Self::is_patch_volume(&second.path))
        .then_with(|| first.path.cmp(&second.path))
    });
  }

  /// Whether a volume sits inside a `patches` directory.
  ///
  /// Matched as a path component rather than a substring, so a directory merely containing the word — `mypatches`, a
  /// user folder named `patches_backup` — does not get patch priority. Component comparison also works for non-UTF-8
  /// paths, where the previous string conversion panicked.
  fn is_patch_volume(path: &Path) -> bool {
    path
      .components()
      .any(|component| component.as_os_str().eq_ignore_ascii_case("patches"))
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
  fn patches_sort_last_by_component_rather_than_by_substring() {
    // Later wins the name-table merge, so `patches` must sort last — and only a real `patches` directory counts,
    // not a name that merely contains the word.
    let mut archives = [
      archive("/game/db/patches/xpatch_1.db"),
      archive("/game/db/mypatches_mod/data.db0"),
      archive("/game/db/configs.db0"),
    ];

    ArchiveProject::sort_archives(&mut archives);

    let order: Vec<&str> = archives.iter().map(|it| it.path.to_str().expect("utf-8")).collect();

    assert_eq!(
      order,
      vec![
        "/game/db/configs.db0",
        "/game/db/mypatches_mod/data.db0",
        "/game/db/patches/xpatch_1.db",
      ]
    );
  }

  #[test]
  fn recognizes_volume_extensions_without_case() {
    assert!(ArchiveDescriptor::is_valid_db_path(&Path::new("game.db0")));
    assert!(ArchiveDescriptor::is_valid_db_path(&Path::new("GAME.DB0")));
    assert!(ArchiveDescriptor::is_valid_db_path(&Path::new("mod.xdb1")));
    assert!(!ArchiveDescriptor::is_valid_db_path(&Path::new("readme.txt")));
    assert!(!ArchiveDescriptor::is_valid_db_path(&Path::new("noextension")));
  }

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

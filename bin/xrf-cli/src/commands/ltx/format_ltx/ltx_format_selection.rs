use std::collections::HashSet;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};
use xrf_error::{XrfError, XrfResult};
use xrf_ltx::LTX_EXTENSION;
use xrf_vfs::{XrayAsset, XrayAssetContainer, XrayScope, XrayVfs};

use crate::commands::ltx::ltx_installation::mount_installation;

/// The files a formatting run will touch, and what it declined to.
pub struct LtxFormatSelection {
  /// De-duplicated physical paths of loose files to format.
  pub files: Vec<PathBuf>,
  /// Display descriptions of archived winning configs that cannot be formatted in place.
  pub declined: Vec<String>,
}

impl LtxFormatSelection {
  /// Selects LTX files from explicit paths or declared game installations.
  ///
  /// A path directly holding `fsgame.ltx` resolves through the VFS, so loose winning configs are formatted and archived
  /// winners are reported as declined. Other paths use a filesystem walk.
  ///
  /// # Errors
  ///
  /// Returns an error when a path does not exist, cannot be walked, or an installation's `fsgame.ltx` cannot be read,
  /// decoded, or parsed.
  pub fn select(paths: &[&PathBuf]) -> XrfResult<Self> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut declined: Vec<String> = Vec::new();
    let mut visited: HashSet<PathBuf> = HashSet::new();

    for path in paths {
      match mount_installation(path)? {
        Some(vfs) => Self::select_installation(&vfs, &mut files, &mut declined, &mut visited)?,
        None => Self::select_path(path, &mut files, &mut visited)?,
      }
    }

    declined.sort();

    Ok(Self { declined, files })
  }

  /// Returns `true` when neither loose nor archived configs were selected.
  pub fn is_empty(&self) -> bool {
    self.files.is_empty() && self.declined.is_empty()
  }

  /// Selects an installation's winning LTX entries.
  ///
  /// Loose winners are formatted through their physical paths. Archived winners are declined because archive volumes
  /// cannot be rewritten in place.
  fn select_installation(
    vfs: &XrayVfs,
    files: &mut Vec<PathBuf>,
    declined: &mut Vec<String>,
    visited: &mut HashSet<PathBuf>,
  ) -> XrfResult<()> {
    for location in vfs.entries(&XrayScope::all()) {
      if !Self::is_ltx(location.logical_path()) {
        continue;
      }

      match location.container() {
        XrayAssetContainer::Directory { .. } => {
          if let Some(physical) = location.physical_path()
            && visited.insert(physical.clone())
          {
            files.push(physical);
          }
        }
        XrayAssetContainer::Archive { .. } => declined.push(Self::describe_declined(&location)),
      }
    }

    files.sort();

    Ok(())
  }

  /// Expands one path the way the command always has: a directory walked for `*.ltx`, a file taken as given.
  ///
  /// An explicitly named file is taken whatever its extension, so a caller can format an arbitrary subset.
  fn select_path(path: &Path, files: &mut Vec<PathBuf>, visited: &mut HashSet<PathBuf>) -> XrfResult<()> {
    if path.is_dir() {
      for entry in WalkDir::new(path) {
        let entry: DirEntry = entry.map_err(|error| error.into_io_error().unwrap())?;
        let entry_path: &Path = entry.path();

        if entry_path.is_file()
          && entry_path
            .extension()
            .is_some_and(|extension| extension == LTX_EXTENSION)
          && visited.insert(entry_path.into())
        {
          files.push(entry_path.into());
        }
      }

      return Ok(());
    }

    if !path.exists() {
      return Err(XrfError::new_not_found_error(format!(
        "Failed to format ltx, provided path does not exist: {}",
        path.display()
      )));
    }

    if visited.insert(path.to_path_buf()) {
      files.push(path.to_path_buf());
    }

    Ok(())
  }

  fn is_ltx(logical_path: &str) -> bool {
    logical_path.ends_with(&format!(".{LTX_EXTENSION}"))
  }

  fn describe_declined(location: &XrayAsset) -> String {
    format!("{} in {}", location.logical_path(), location.describe_container())
  }
}

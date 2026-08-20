use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use xrf_error::{XrfError, XrfResult};

use crate::source::DirectoryAsset;

#[derive(Debug)]
pub struct DirectoryAssetIndex {
  root: PathBuf,
  assets: Vec<DirectoryAsset>,
}

impl DirectoryAssetIndex {
  /// Recursively indexes files below `root` in relative-path order.
  ///
  /// Directory paths and symbolic links to directories are not added as assets.
  ///
  /// An entry the walk cannot read — a permission-denied subdirectory, a broken link — is warned about and skipped
  /// rather than failing the index. Aborting would discard the whole mount over one unreadable corner of a tree, and the
  /// mount being absent then reads downstream as content that is missing.
  ///
  /// # Errors
  ///
  /// Returns an error when a file path cannot be made relative to `root`, which would mean the walk left its own root.
  pub fn read(root: impl AsRef<Path>) -> XrfResult<Self> {
    let root: &Path = root.as_ref();

    log::debug!("reading directory assets from {}", root.display());

    let mut assets: Vec<DirectoryAsset> = Vec::new();

    for entry in WalkDir::new(root).follow_links(false) {
      let entry = match entry {
        Ok(entry) => entry,
        Err(error) => {
          log::warn!("Skipping unreadable directory entry under {}: {error}", root.display());

          continue;
        }
      };

      if !entry.file_type().is_file() {
        continue;
      }

      let relative_path = entry
        .path()
        .strip_prefix(root)
        .map_err(|_| XrfError::new_unexpected_error("failed to strip directory asset root"))?
        .to_path_buf();
      assets.push(DirectoryAsset::new(relative_path));
    }

    assets.sort_by(|left, right| left.relative_path().cmp(right.relative_path()));

    log::debug!("read {} directory assets from {}", assets.len(), root.display());

    Ok(Self {
      root: root.to_path_buf(),
      assets,
    })
  }

  /// Returns the root from which relative paths are measured.
  pub fn root(&self) -> &Path {
    &self.root
  }

  /// Iterates over all indexed files in relative-path order.
  pub fn assets(&self) -> impl Iterator<Item = &DirectoryAsset> {
    self.assets.iter()
  }

  /// Finds an exact root-relative filesystem path.
  pub fn find(&self, relative_path: &Path) -> Option<&DirectoryAsset> {
    self.assets.iter().find(|asset| asset.relative_path() == relative_path)
  }

  /// Iterates over assets below a root-relative path prefix.
  pub fn with_prefix(&self, prefix: &Path) -> impl Iterator<Item = &DirectoryAsset> {
    self
      .assets
      .iter()
      .filter(move |asset| asset.relative_path().starts_with(prefix))
  }

  /// Iterates over assets whose filesystem extension equals `extension`.
  pub fn with_extension(&self, extension: &OsStr) -> impl Iterator<Item = &DirectoryAsset> {
    self
      .assets
      .iter()
      .filter(move |asset| asset.relative_path().extension() == Some(extension))
  }

  pub(crate) fn asset(&self, index: usize) -> &DirectoryAsset {
    &self.assets[index]
  }
}

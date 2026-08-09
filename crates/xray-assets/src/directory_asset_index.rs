use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use xray_error::{XRayError, XRayResult};

use crate::DirectoryAsset;

#[derive(Debug)]
pub struct DirectoryAssetIndex {
  root: PathBuf,
  assets: Vec<DirectoryAsset>,
}

impl DirectoryAssetIndex {
  pub fn read(root: impl AsRef<Path>) -> XRayResult<Self> {
    let root: &Path = root.as_ref();

    log::debug!("reading directory assets from {}", root.display());

    let mut assets: Vec<DirectoryAsset> = Vec::new();

    for entry in WalkDir::new(root).follow_links(false) {
      let entry =
        entry.map_err(|error| XRayError::new_asset_error(format!("failed to read directory asset entry: {error}")))?;

      if !entry.file_type().is_file() {
        continue;
      }

      let relative_path = entry
        .path()
        .strip_prefix(root)
        .map_err(|_| XRayError::new_unexpected_error("failed to strip directory asset root"))?
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

  pub fn root(&self) -> &Path {
    &self.root
  }

  pub fn assets(&self) -> impl Iterator<Item = &DirectoryAsset> {
    self.assets.iter()
  }

  pub fn find(&self, relative_path: &Path) -> Option<&DirectoryAsset> {
    self.assets.iter().find(|asset| asset.relative_path() == relative_path)
  }

  pub fn with_prefix(&self, prefix: &Path) -> impl Iterator<Item = &DirectoryAsset> {
    self
      .assets
      .iter()
      .filter(move |asset| asset.relative_path().starts_with(prefix))
  }

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

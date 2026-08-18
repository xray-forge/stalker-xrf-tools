use std::path::{Path, PathBuf};

use crate::{DirectoryAsset, XrayAssetType};

/// A borrowed asset carrying both its canonical X-Ray path and physical directory entry.
#[derive(Debug, Clone, Copy)]
pub struct XrayAsset<'a> {
  pub(crate) logical_path: &'a str,
  pub(crate) asset_type: Option<XrayAssetType>,
  pub(crate) directory_asset: &'a DirectoryAsset,
  pub(crate) root: &'a Path,
}

impl XrayAsset<'_> {
  /// Returns the extension-derived asset type, when the path is recognized.
  pub fn asset_type(&self) -> Option<XrayAssetType> {
    self.asset_type
  }

  /// Returns whether this asset has the requested extension-derived type.
  pub fn is_type(&self, asset_type: XrayAssetType) -> bool {
    self.asset_type == Some(asset_type)
  }

  /// Returns the canonical lower-case, backslash-separated path used by the X-Ray engine.
  ///
  /// Unlike [`Self::relative_path`], this is an engine identity rather than a physical filesystem path.
  pub fn logical_path(&self) -> &str {
    self.logical_path
  }

  /// Returns this asset's root-relative physical filesystem path.
  ///
  /// Use [`Self::absolute_path`] when a filesystem operation needs the indexed root as well.
  pub fn relative_path(&self) -> &Path {
    self.directory_asset.relative_path()
  }

  /// Resolves this asset's physical path from the indexed root.
  pub fn absolute_path(&self) -> PathBuf {
    self.root.join(self.relative_path())
  }
}

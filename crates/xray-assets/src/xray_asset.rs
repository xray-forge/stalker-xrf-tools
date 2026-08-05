use crate::{DirectoryAsset, XrayAssetType};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub struct XrayAsset<'a> {
  pub(crate) logical_path: &'a str,
  pub(crate) asset_type: Option<XrayAssetType>,
  pub(crate) directory_asset: &'a DirectoryAsset,
  pub(crate) root: &'a Path,
}

impl XrayAsset<'_> {
  pub fn logical_path(&self) -> &str {
    self.logical_path
  }

  pub fn asset_type(&self) -> Option<XrayAssetType> {
    self.asset_type
  }

  pub fn is_type(&self, asset_type: XrayAssetType) -> bool {
    self.asset_type == Some(asset_type)
  }

  pub fn relative_path(&self) -> &Path {
    self.directory_asset.relative_path()
  }

  pub fn absolute_path(&self) -> PathBuf {
    self.root.join(self.relative_path())
  }
}

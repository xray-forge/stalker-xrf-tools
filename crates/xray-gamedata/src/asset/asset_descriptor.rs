use crate::asset::asset_type::AssetType;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct AssetDescriptor {
  pub asset_type: AssetType,
  pub relative_path: PathBuf,
}

impl AssetDescriptor {
  pub fn from_relative_path<P: AsRef<Path>>(relative_path: P) -> Self {
    let relative_path: &Path = relative_path.as_ref();
    let asset_type: AssetType = AssetType::from_path(&relative_path.to_string_lossy());

    Self {
      asset_type,
      relative_path: relative_path.to_path_buf(),
    }
  }
}

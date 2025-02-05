use crate::asset::asset_type::AssetType;

#[derive(Debug, Clone, PartialEq)]
pub struct AssetDescriptor {
  pub root_index: usize,
  pub asset_type: AssetType,
}

impl AssetDescriptor {
  pub fn new(root_index: usize, asset_type: AssetType) -> Self {
    Self {
      root_index,
      asset_type,
    }
  }

  pub fn new_with_extension(root_index: usize, relative_path: &str) -> Self {
    let extension: AssetType = AssetType::from_path(relative_path);

    Self {
      root_index,
      asset_type: extension,
    }
  }
}

use crate::asset::asset_type::AssetType;

#[derive(Debug, Clone, PartialEq)]
pub struct AssetDescriptor {
  pub asset_type: AssetType,
}

impl AssetDescriptor {
  pub fn new(asset_type: AssetType) -> Self {
    Self { asset_type }
  }

  pub fn new_with_extension(relative_path: &str) -> Self {
    let extension: AssetType = AssetType::from_path(relative_path);

    Self {
      asset_type: extension,
    }
  }
}

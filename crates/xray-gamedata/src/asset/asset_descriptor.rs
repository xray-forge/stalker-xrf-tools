use crate::asset::asset_type::AssetType;

#[derive(Debug, Clone, PartialEq)]
pub struct AssetDescriptor {
  pub root_index: usize,
  pub hits: usize,
  pub asset_type: AssetType,
}

impl AssetDescriptor {
  pub fn new(root_index: usize) -> Self {
    Self {
      root_index,
      hits: 0,
      asset_type: AssetType::Unknown,
    }
  }

  pub fn new_with_extension(root_index: usize, relative_path: &str) -> Self {
    let extension: AssetType = AssetType::from_path(relative_path);

    if extension == AssetType::Unknown {
      log::warn!("Unknown extension asset: {}", relative_path);
    }

    Self {
      root_index,
      hits: 0,
      asset_type: extension,
    }
  }
}

impl AssetDescriptor {
  pub fn add_hit(&mut self) {
    self.hits += 1;
  }
}

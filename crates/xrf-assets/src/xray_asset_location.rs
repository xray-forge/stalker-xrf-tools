use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::XrayAsset;

/// Where an asset was found, detached from the index that found it.
///
/// [`XrayAsset`] borrows from its index, so it cannot be parked in state, sent over IPC or kept past the lookup. This is
/// the same three facts owned: which root answered, which file inside it, and the engine identity it answers to.
///
/// Root and relative path stay separate rather than joined, because "which root did this come from" is the question an
/// overlay tree makes interesting, and joining them throws it away.
///
/// When archive-backed assets arrive this is the type that gains a container, so a consumer reading a located asset does
/// not change shape when the bytes start coming out of a `.db`.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct XrayAssetLocation {
  /// Indexed root this resolved against.
  root: PathBuf,
  /// Physical path inside that root.
  relative_path: PathBuf,
  /// Lower-case, backslash-separated engine identity.
  logical_path: String,
}

impl XrayAssetLocation {
  pub fn root(&self) -> &Path {
    &self.root
  }

  pub fn relative_path(&self) -> &Path {
    &self.relative_path
  }

  pub fn logical_path(&self) -> &str {
    &self.logical_path
  }

  /// Physical path to read bytes from.
  pub fn absolute_path(&self) -> PathBuf {
    self.root.join(&self.relative_path)
  }
}

impl From<XrayAsset<'_>> for XrayAssetLocation {
  fn from(asset: XrayAsset<'_>) -> Self {
    Self {
      root: asset.root.to_path_buf(),
      relative_path: asset.relative_path().to_path_buf(),
      logical_path: asset.logical_path().to_string(),
    }
  }
}

impl XrayAsset<'_> {
  /// This asset as an owned record, for storing or sending.
  pub fn into_location(self) -> XrayAssetLocation {
    XrayAssetLocation::from(self)
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::{Path, PathBuf};

  use xrf_test_utils::utils::get_absolute_generated_test_resource_path;

  use crate::{DirectoryAssetIndex, XrayAssetIndex, XrayAssetLocation};

  #[test]
  fn keeps_the_root_and_the_engine_identity_apart_from_the_physical_path() {
    let root: PathBuf = get_absolute_generated_test_resource_path("xray_asset_location/tree");

    let _ = fs::remove_dir_all(&root);

    fs::create_dir_all(root.join("textures/wpn")).expect("test tree is creatable");
    fs::write(root.join("textures/wpn/wpn_ak74.dds"), []).expect("test texture is writable");

    let index: XrayAssetIndex =
      XrayAssetIndex::new(DirectoryAssetIndex::read(&root).expect("root walks"), &[]).expect("root indexes");

    let location: XrayAssetLocation = index
      .dds_texture("wpn\\wpn_ak74")
      .expect("lookup succeeds")
      .expect("texture is found")
      .into_location();

    assert_eq!(location.root(), root.as_path());
    assert_eq!(location.relative_path(), Path::new("textures").join("wpn/wpn_ak74.dds"));
    assert_eq!(location.logical_path(), "textures\\wpn\\wpn_ak74.dds");
    assert_eq!(location.absolute_path(), root.join("textures/wpn/wpn_ak74.dds"));
  }
}

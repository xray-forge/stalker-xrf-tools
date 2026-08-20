use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::XrayAssetType;
use crate::xray_path::XrayPath;

/// The physical container of a located asset.
///
/// Separate variants prevent callers from treating an archived entry as a loose file with a usable filesystem path.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
// `rename_all_fields` keeps struct-variant fields camel-cased alongside the variants.
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum XrayAssetContainer {
  /// A loose file, preserving its root so consumers can identify the winning overlay.
  Directory { root: PathBuf, relative_path: PathBuf },
  /// An entry inside the archive volume set at `path`.
  Archive { path: PathBuf },
}

/// One asset a mount resolved: its engine identity plus the container it came out of.
///
/// Owned rather than borrowed, so it can be stored, sorted or sent over IPC — which is what an editor that mounts and
/// writes needs, and why nothing borrowed reaches past this crate.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct XrayAsset {
  /// Lower-case, backslash-separated engine identity, including the mount's logical base.
  logical_path: XrayPath,
  /// Physical container reported by the source that resolved the asset.
  container: XrayAssetContainer,
}

impl XrayAsset {
  /// Creates a location for a loose file.
  ///
  /// `relative_path` is joined to `root` only when [`Self::physical_path`] is requested; the
  /// logical path remains the normalized X-Ray identity used for lookups and IPC.
  pub fn new_directory(logical_path: XrayPath, root: PathBuf, relative_path: PathBuf) -> Self {
    Self {
      container: XrayAssetContainer::Directory { relative_path, root },
      logical_path,
    }
  }

  /// Creates a location for an entry in an archive volume set.
  ///
  /// The path identifies the archive container, not a filesystem path for the entry itself.
  pub fn new_archive(logical_path: XrayPath, path: PathBuf) -> Self {
    Self {
      container: XrayAssetContainer::Archive { path },
      logical_path,
    }
  }

  /// Creates a location from an engine path and a source-reported container.
  ///
  /// The caller is responsible for passing the normalized logical path returned by the VFS.
  pub fn new(logical_path: XrayPath, container: XrayAssetContainer) -> Self {
    Self {
      container,
      logical_path,
    }
  }

  /// Returns the normalized X-Ray path, including any mount base.
  pub fn logical_path(&self) -> &XrayPath {
    &self.logical_path
  }

  /// Returns the physical container that supplied this location.
  pub fn container(&self) -> &XrayAssetContainer {
    &self.container
  }

  /// Returns the kind this asset's extension identifies, when it is one the tools recognize.
  ///
  /// Derived from the logical path rather than stored, because the path is the only evidence: a container says where the
  /// bytes are, not what they mean.
  pub fn asset_type(&self) -> Option<XrayAssetType> {
    XrayAssetType::from_logical_path(self.logical_path.as_str())
  }

  /// Whether this asset's extension identifies the requested kind.
  pub fn is_type(&self, asset_type: XrayAssetType) -> bool {
    self.asset_type() == Some(asset_type)
  }

  /// Returns the containing tree for a loose asset, or `None` for an archived asset.
  pub fn root(&self) -> Option<&Path> {
    match &self.container {
      XrayAssetContainer::Directory { root, .. } => Some(root),
      XrayAssetContainer::Archive { .. } => None,
    }
  }

  /// Returns a readable filesystem path for a loose asset.
  ///
  /// Archived assets return `None`; callers that support both containers should read through [`crate::XrayVfs`].
  pub fn physical_path(&self) -> Option<PathBuf> {
    match &self.container {
      XrayAssetContainer::Directory { relative_path, root } => Some(root.join(relative_path)),
      XrayAssetContainer::Archive { .. } => None,
    }
  }

  /// Describes the containing tree or archive volume set for display.
  pub fn describe_container(&self) -> String {
    match &self.container {
      XrayAssetContainer::Directory { root, .. } => root.display().to_string(),
      XrayAssetContainer::Archive { path } => format!("{} (archive)", path.display()),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};

  use crate::{XrayAsset, XrayAssetContainer, XrayPath};

  #[test]
  fn a_directory_asset_answers_a_physical_path() {
    // Host paths are built from components, never from a `\`-joined literal: `PathBuf::join` inserts the platform separator,
    // so a Windows-shaped literal compares unequal to a joined path on Linux while passing here.
    let root: PathBuf = PathBuf::from("gamedata");
    let relative: PathBuf = Path::new("textures").join("wpn").join("wpn_ak74.dds");
    let asset: XrayAsset = XrayAsset::new_directory(
      XrayPath::new("textures\\wpn\\wpn_ak74.dds").expect("valid logical path"),
      root.clone(),
      relative.clone(),
    );

    assert_eq!(asset.root(), Some(root.as_path()));
    assert_eq!(
      asset.physical_path(),
      Some(root.join("textures").join("wpn").join("wpn_ak74.dds"))
    );
    assert_eq!(
      asset.logical_path().as_str(),
      "textures\\wpn\\wpn_ak74.dds",
      "the engine identity keeps backslashes on every platform, unlike the host path beside it"
    );
  }

  #[test]
  fn an_archived_asset_answers_no_physical_path_rather_than_a_plausible_one() {
    // Archive entries have no physical path; joining the volume directory to the logical path would invent one.
    let location: XrayAsset = XrayAsset::new_archive(
      XrayPath::new("textures\\wpn\\wpn_ak74.dds").expect("valid logical path"),
      Path::new("anomaly").join("db").join("textures"),
    );

    assert_eq!(location.root(), None);
    assert_eq!(location.physical_path(), None);
    assert!(matches!(location.container(), XrayAssetContainer::Archive { .. }));
    assert!(location.describe_container().ends_with("(archive)"));
  }
}

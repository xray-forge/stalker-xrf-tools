use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::XrayAsset;

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

/// An owned result of locating an asset.
///
/// Unlike `XrayAsset`, this type can be stored or sent over IPC. It preserves the engine path and source-reported
/// container for either a loose or archived asset.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct XrayAssetLocation {
  /// Lower-case, backslash-separated engine identity, including the mount's logical base.
  logical_path: String,
  /// Physical container reported by the source that resolved the asset.
  container: XrayAssetContainer,
}

impl XrayAssetLocation {
  pub fn new_directory(logical_path: String, root: PathBuf, relative_path: PathBuf) -> Self {
    Self {
      container: XrayAssetContainer::Directory { relative_path, root },
      logical_path,
    }
  }

  pub fn new_archive(logical_path: String, path: PathBuf) -> Self {
    Self {
      container: XrayAssetContainer::Archive { path },
      logical_path,
    }
  }

  /// Creates a location from an engine path and a source-reported container.
  pub fn new(logical_path: String, container: XrayAssetContainer) -> Self {
    Self {
      container,
      logical_path,
    }
  }

  pub fn logical_path(&self) -> &str {
    &self.logical_path
  }

  pub fn container(&self) -> &XrayAssetContainer {
    &self.container
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

impl XrayAsset<'_> {
  /// Converts this borrowed asset into an owned location.
  pub fn into_location(self) -> XrayAssetLocation {
    XrayAssetLocation::new_directory(
      self.logical_path().to_string(),
      self.root.to_path_buf(),
      self.relative_path().to_path_buf(),
    )
  }
}

#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};

  use crate::{XrayAssetContainer, XrayAssetLocation};

  #[test]
  fn a_directory_asset_answers_a_physical_path() {
    let location: XrayAssetLocation = XrayAssetLocation::new_directory(
      String::from("textures\\wpn\\wpn_ak74.dds"),
      PathBuf::from("C:\\gamedata"),
      PathBuf::from("textures\\wpn\\wpn_ak74.dds"),
    );

    assert_eq!(location.root(), Some(Path::new("C:\\gamedata")));
    assert_eq!(
      location.physical_path(),
      Some(PathBuf::from("C:\\gamedata\\textures\\wpn\\wpn_ak74.dds"))
    );
  }

  #[test]
  fn an_archived_asset_answers_no_physical_path_rather_than_a_plausible_one() {
    // Archive entries have no physical path; joining the volume directory to the logical path would invent one.
    let location: XrayAssetLocation = XrayAssetLocation::new_archive(
      String::from("textures\\wpn\\wpn_ak74.dds"),
      PathBuf::from("C:\\anomaly\\db\\textures"),
    );

    assert_eq!(location.root(), None);
    assert_eq!(location.physical_path(), None);
    assert!(matches!(location.container(), XrayAssetContainer::Archive { .. }));
    assert!(location.describe_container().ends_with("(archive)"));
  }
}

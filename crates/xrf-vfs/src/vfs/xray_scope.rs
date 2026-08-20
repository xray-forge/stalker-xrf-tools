use xrf_error::XrfResult;

use crate::path::normalize;
use crate::{XrayMount, XrayMountId, XrayMountKind};

/// A mount filter for one VFS operation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum XrayMountSelection {
  /// Every mount, in mount order.
  #[default]
  All,
  /// Only the named mounts, still visited in mount order rather than in the order given.
  Only(Vec<XrayMountId>),
  /// Only mounts that can be written to, which excludes every archive.
  Writable,
  /// Only mounts of one kind.
  OfKind(XrayMountKind),
}

/// A mount selection optionally restricted to one logical subtree.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct XrayScope {
  selection: XrayMountSelection,
  prefix: Option<String>,
}

impl XrayScope {
  /// Selects every mount in priority order.
  pub fn all() -> Self {
    Self::default()
  }

  /// Selects only writable mounts.
  pub fn writable() -> Self {
    Self {
      prefix: None,
      selection: XrayMountSelection::Writable,
    }
  }

  /// Selects the named mounts while preserving VFS priority order.
  pub fn only(mounts: impl IntoIterator<Item = XrayMountId>) -> Self {
    Self {
      prefix: None,
      selection: XrayMountSelection::Only(mounts.into_iter().collect()),
    }
  }

  /// Selects mounts of one storage kind.
  pub fn of_kind(kind: XrayMountKind) -> Self {
    Self {
      prefix: None,
      selection: XrayMountSelection::OfKind(kind),
    }
  }

  /// Restricts this scope to a normalized logical subtree such as `configs` or `textures\wpn`.
  ///
  /// # Errors
  ///
  /// Returns an error when `prefix` is not a valid X-Ray logical path.
  pub fn with_prefix(mut self, prefix: &str) -> XrfResult<Self> {
    self.prefix = Some(normalize(prefix)?);

    Ok(self)
  }

  pub(crate) fn selection(&self) -> &XrayMountSelection {
    &self.selection
  }

  /// Returns the normalized logical subtree restriction, if any.
  pub fn prefix(&self) -> Option<&str> {
    self.prefix.as_deref()
  }

  /// Checks whether a mount matches this scope's selection.
  pub fn includes(&self, mount: &XrayMount) -> bool {
    match &self.selection {
      XrayMountSelection::All => true,
      XrayMountSelection::Only(ids) => ids.contains(&mount.id()),
      XrayMountSelection::Writable => mount.is_writable(),
      XrayMountSelection::OfKind(kind) => mount.kind() == *kind,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::XrayMountSelection;
  use crate::{XrayMountKind, XrayScope};

  #[test]
  fn defaults_to_everything() {
    assert_eq!(XrayScope::all().selection(), &XrayMountSelection::All);
    assert_eq!(XrayScope::all().prefix(), None);
  }

  #[test]
  fn normalizes_a_prefix_the_way_a_logical_path_is_normalized() {
    let scope: XrayScope = XrayScope::all()
      .with_prefix("Configs/Weapons")
      .expect("prefix is valid");

    assert_eq!(scope.prefix(), Some("configs\\weapons"));
  }

  #[test]
  fn rejects_an_ambiguous_prefix() {
    assert!(XrayScope::all().with_prefix("configs/../textures").is_err());
  }

  #[test]
  fn selects_by_kind() {
    assert_eq!(
      XrayScope::of_kind(XrayMountKind::Archive).selection(),
      &XrayMountSelection::OfKind(XrayMountKind::Archive)
    );
  }
}

use std::borrow::Cow;

use serde::Serialize;
use xrf_error::XrfResult;

use crate::path::{is_component_prefix, join, normalize};
use crate::{XrayAssetSource, XrayMountKind};

/// Stable identity of a mount within one VFS.
///
/// Labels need not be unique, so scopes select mounts by this identifier.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct XrayMountId(pub(crate) usize);

/// One source mounted at a logical base.
///
/// The base maps source-relative paths into the engine namespace. An empty base mounts a complete root; a base such as
/// `configs\weapons` mounts only that logical subtree.
#[derive(Debug)]
pub struct XrayMount {
  id: XrayMountId,
  base: String,
  source: Box<dyn XrayAssetSource>,
}

impl XrayMount {
  /// Creates a mount at a logical base, using an empty base for a complete root.
  ///
  /// # Errors
  ///
  /// Returns an error when a non-empty base is not a valid X-Ray logical path.
  pub fn new(id: XrayMountId, base: &str, source: Box<dyn XrayAssetSource>) -> XrfResult<Self> {
    Ok(Self {
      base: if base.is_empty() {
        String::new()
      } else {
        normalize(base)?.into_owned()
      },
      id,
      source,
    })
  }

  pub fn id(&self) -> XrayMountId {
    self.id
  }

  /// Returns the normalized logical base assigned to this mount.
  pub fn base(&self) -> &str {
    &self.base
  }

  /// Returns whether the mount stores loose files or archive entries.
  pub fn kind(&self) -> XrayMountKind {
    self.source.kind()
  }

  /// Returns whether writes through this mount can update existing entries.
  pub fn is_writable(&self) -> bool {
    self.source.is_writable()
  }

  /// Returns the source label used in diagnostics.
  pub fn label(&self) -> &str {
    self.source.label()
  }

  /// Borrows the mounted source for source-specific inspection.
  pub fn source(&self) -> &dyn XrayAssetSource {
    self.source.as_ref()
  }

  /// Converts a logical path to a source-relative path, or returns `None` when it lies outside the mount's base.
  ///
  /// Borrowed for the common case: most mounts sit at the logical root, where the source path *is* the logical path, and
  /// this runs once per mount on every lookup — allocating a copy of each probed path was the cost of saying nothing.
  pub(crate) fn to_source_path<'a>(&self, logical_path: &'a str) -> Option<Cow<'a, str>> {
    if self.base.is_empty() {
      return Some(Cow::Borrowed(logical_path));
    }

    if !is_component_prefix(logical_path, &self.base) {
      return None;
    }

    Some(Cow::Borrowed(logical_path[self.base.len()..].trim_start_matches('\\')))
  }

  /// Applies this mount's base to a source-relative path.
  ///
  /// Borrowed for a root mount, where the source path already *is* the logical path — enumeration calls this once per
  /// entry, so copying each one to say nothing was the bulk of its allocation.
  pub(crate) fn to_logical_path<'a>(&self, source_path: &'a str) -> XrfResult<Cow<'a, str>> {
    if self.base.is_empty() {
      return normalize(source_path);
    }

    Ok(Cow::Owned(join(&self.base, source_path)?))
  }

  /// Translates a scope prefix into the source's namespace.
  ///
  /// Returns `None` when the scope cannot overlap this mount, and `Some(None)` when every source entry qualifies.
  pub(crate) fn to_source_prefix(&self, logical_prefix: Option<&str>) -> Option<Option<String>> {
    let Some(prefix) = logical_prefix else {
      return Some(None);
    };

    if self.base.is_empty() {
      return Some(Some(prefix.to_string()));
    }

    // The prefix reaches into this mount, so narrow it to the part below the base.
    if let Some(inner) = self.to_source_path(prefix) {
      return Some(if inner.is_empty() {
        None
      } else {
        Some(inner.into_owned())
      });
    }

    // The base sits inside the requested prefix, so the whole mount qualifies.
    if is_component_prefix(&self.base, prefix) {
      return Some(None);
    }

    None
  }
}

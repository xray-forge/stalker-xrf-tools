use std::fmt::Debug;
use std::path::Path;

use serde::Serialize;
use xrf_error::XrfResult;

use crate::XrayAssetContainer;

/// The storage kind backing a mount.
///
/// It distinguishes loose filesystem entries from entries inside archive volumes.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum XrayMountKind {
  /// Loose files under a directory.
  Directory,
  /// Entries inside a set of `.db` archive volumes.
  Archive,
}

/// An asset source addressed by normalized paths relative to itself.
///
/// [`crate::XrayMount`] strips its logical base before calling the source, allowing the same source type to back a root or
/// a subtree. Sources are `Send + Sync` because the mounted VFS is shared across application commands.
pub trait XrayAssetSource: Debug + Send + Sync {
  /// Short name for reporting, such as a directory or volume-set name.
  fn label(&self) -> &str;

  /// Classifies the source's physical storage.
  fn kind(&self) -> XrayMountKind;

  /// Whether existing entries can be overwritten through [`Self::write`].
  fn is_writable(&self) -> bool;

  /// Returns the directory root or the directory containing the archive volumes.
  fn root_path(&self) -> &Path;

  /// Checks whether the source contains a source-relative logical path.
  fn contains(&self, path: &str) -> bool;

  /// Locates an entry in its physical container.
  ///
  /// Returns `None` when the source does not contain `path`.
  fn locate(&self, path: &str) -> Option<XrayAssetContainer>;

  /// Reads an existing entry.
  fn read(&self, path: &str) -> XrfResult<Vec<u8>>;

  /// Overwrites an existing entry when [`Self::is_writable`] is true.
  fn write(&self, path: &str, bytes: &[u8]) -> XrfResult<()>;

  /// Creates an entry the source does not currently expose, when writable.
  ///
  /// Implementations may leave mount-time indexes stale. [`crate::XrayVfs::write_override`] remounts after creation.
  fn create(&self, path: &str, bytes: &[u8]) -> XrfResult<()>;

  /// Enumerates source-relative logical paths, optionally restricted to a component prefix.
  fn entries<'a>(&'a self, prefix: Option<&'a str>) -> Box<dyn Iterator<Item = String> + 'a>;
}

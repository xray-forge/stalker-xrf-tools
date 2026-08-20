use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::Path;

use xrf_error::{XrfError, XrfResult};

use crate::archive::project::archive_project::ArchiveProject;
use crate::path::{is_component_prefix, normalize_logical};
use crate::{XrayAssetContainer, XrayAssetSource, XrayMountKind};

/// Mounts an archive volume set as a read-only asset source.
///
/// Directory paths are scanned nonrecursively, matching `recurs = false` archive aliases and avoiding duplicate
/// subdirectory mounts.
///
/// [`ArchiveProject`] already merges a volume set into one name table with the later volume winning, which matches how the
/// engine registers them, so this adds only the logical-path keying a VFS lookup needs.
pub struct ArchiveAssetSource {
  label: String,
  project: ArchiveProject,
  /// Normalized logical path to the key `project.files` stores.
  ///
  /// Archive headers keep names as authored, so the normalized form is derived once here rather than per lookup.
  entries: HashMap<String, String>,
}

impl ArchiveAssetSource {
  /// Opens a volume set, or a single volume, at a path.
  pub fn read(path: impl AsRef<Path>) -> XrfResult<Self> {
    let path: &Path = path.as_ref();
    let project: ArchiveProject = ArchiveProject::new_shallow(&path)?;

    let entries: HashMap<String, String> = project
      .files
      .iter()
      .filter(|(name, descriptor)| !is_directory_entry(name, descriptor.size_real))
      .filter_map(|(name, _)| {
        normalize_logical(name)
          .inspect_err(|error| log::warn!("Skipping archive entry '{name}': {error}"))
          .ok()
          .map(|normalized| (normalized, name.clone()))
      })
      .collect();

    log::info!("Mounted {} archive entries from {}", entries.len(), path.display());

    Ok(Self {
      entries,
      label: path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string()),
      project,
    })
  }

  pub fn project(&self) -> &ArchiveProject {
    &self.project
  }

  pub fn entry_count(&self) -> usize {
    self.entries.len()
  }
}

/// Written by hand rather than derived, because a derived one would print the whole name table - 17,188 assets for
/// Anomaly's texture volumes alone. What identifies a mount is which volume set it is and how much it holds.
impl Debug for ArchiveAssetSource {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("ArchiveAssetSource")
      .field("label", &self.label)
      .field("root", &self.project.root)
      .field("entries", &self.entries.len())
      .finish()
  }
}

/// Whether an archive entry names a directory rather than an asset.
///
/// A volume records the directories it contains so an unpacker can recreate them, and those entries would otherwise become
/// phantom assets: `contains("configs")` would answer true and enumeration would count four extras for a three-file tree.
/// The rule matches `ArchiveUnpacker::extract_directory`, which skips on the same two conditions, so an entry the unpacker
/// declines to write is one a lookup cannot resolve. A genuinely empty file is skipped too, exactly as it is there.
fn is_directory_entry(name: &str, size_real: u32) -> bool {
  size_real == 0 || name.ends_with(['\\', '/'])
}

impl XrayAssetSource for ArchiveAssetSource {
  fn label(&self) -> &str {
    &self.label
  }

  fn kind(&self) -> XrayMountKind {
    XrayMountKind::Archive
  }

  /// Always false. Writing into a volume is out of scope; a caller wanting to change an archived asset writes a loose
  /// override instead.
  fn is_writable(&self) -> bool {
    false
  }

  fn root_path(&self) -> &Path {
    &self.project.root
  }

  fn contains(&self, path: &str) -> bool {
    self.entries.contains_key(path)
  }

  fn locate(&self, path: &str) -> Option<XrayAssetContainer> {
    self.entries.contains_key(path).then(|| XrayAssetContainer::Archive {
      path: self.project.root.clone(),
    })
  }

  fn read(&self, path: &str) -> XrfResult<Vec<u8>> {
    let Some(name) = self.entries.get(path) else {
      // Absent, not unreadable: the distinction lets a caller fall back rather than fail.
      return Err(XrfError::new_not_found_error(format!(
        "no archive entry '{path}' in {}",
        self.label
      )));
    };

    self.project.read_file_bytes(name)
  }

  /// Answers from the volume's name table, so no entry is decompressed to learn its size.
  fn size(&self, path: &str) -> Option<u64> {
    self
      .entries
      .get(path)
      .and_then(|name| self.project.files.get(name))
      .map(|descriptor| u64::from(descriptor.size_real))
  }

  fn write(&self, path: &str, _bytes: &[u8]) -> XrfResult<()> {
    Err(XrfError::new_read_error(format!(
      "cannot write '{path}': archive '{}' is read only",
      self.label
    )))
  }

  /// Always fails. A volume cannot gain an entry; an override belongs in a loose mount in front of it.
  fn create(&self, path: &str, _bytes: &[u8]) -> XrfResult<()> {
    Err(XrfError::new_read_error(format!(
      "cannot create '{path}': archive '{}' is read only",
      self.label
    )))
  }

  fn entries<'a>(&'a self, prefix: Option<&'a str>) -> Box<dyn Iterator<Item = String> + 'a> {
    Box::new(
      self
        .entries
        .keys()
        .filter(move |path| prefix.is_none_or(|prefix| is_component_prefix(path, prefix)))
        .cloned(),
    )
  }
}

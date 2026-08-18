use std::fs;
use std::path::{Path, PathBuf};

use xrf_error::{XrfError, XrfResult};

use crate::xray_asset_utils::is_component_prefix;
use crate::{DirectoryAssetIndex, XrayAssetContainer, XrayAssetIndex, XrayAssetSource, XrayMountKind};

/// A directory of loose files, indexed once at mount time.
///
/// Its [`XrayAssetIndex`] rejects duplicate logical paths within the directory. Shadowing is allowed only between separate
/// VFS mounts.
#[derive(Debug)]
pub struct XrayDirectorySource {
  label: String,
  index: XrayAssetIndex,
}

impl XrayDirectorySource {
  /// Walks and indexes a directory of loose assets.
  ///
  /// # Errors
  ///
  /// Returns an error when traversal fails, a path is not a valid X-Ray logical path, or two files normalize to the same
  /// logical path.
  pub fn read(root: impl AsRef<Path>) -> XrfResult<Self> {
    let root: &Path = root.as_ref();

    Ok(Self {
      index: XrayAssetIndex::new(DirectoryAssetIndex::read(root)?, &[])?,
      label: root
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| root.display().to_string()),
    })
  }

  pub fn root(&self) -> &Path {
    self.index.root()
  }

  /// Returns the strict logical-path index backing this source.
  pub fn index(&self) -> &XrayAssetIndex {
    &self.index
  }
}

impl XrayAssetSource for XrayDirectorySource {
  fn label(&self) -> &str {
    &self.label
  }

  fn kind(&self) -> XrayMountKind {
    XrayMountKind::Directory
  }

  fn is_writable(&self) -> bool {
    true
  }

  fn root_path(&self) -> &Path {
    self.index.root()
  }

  fn contains(&self, path: &str) -> bool {
    self.index.find(path).ok().flatten().is_some()
  }

  fn locate(&self, path: &str) -> Option<XrayAssetContainer> {
    self
      .index
      .find(path)
      .ok()
      .flatten()
      .map(|asset| XrayAssetContainer::Directory {
        relative_path: asset.relative_path().to_path_buf(),
        root: self.index.root().to_path_buf(),
      })
  }

  fn read(&self, path: &str) -> XrfResult<Vec<u8>> {
    let Some(asset) = self.index.find(path)? else {
      return Err(XrfError::new_asset_error(format!(
        "no asset '{path}' under root {}",
        self.root().display()
      )));
    };

    let absolute: PathBuf = asset.absolute_path();

    fs::read(&absolute)
      .map_err(|error| XrfError::new_asset_error(format!("failed to read '{}': {error}", absolute.display())))
  }

  /// Overwrites an indexed entry without creating new files.
  ///
  /// Refusing absent paths prevents the in-memory index from becoming stale.
  fn write(&self, path: &str, bytes: &[u8]) -> XrfResult<()> {
    let Some(absolute) = self.index.find(path).ok().flatten().map(|asset| asset.absolute_path()) else {
      return Err(XrfError::new_asset_error(format!(
        "no asset '{path}' under root {} to write",
        self.root().display()
      )));
    };

    fs::write(&absolute, bytes)
      .map_err(|error| XrfError::new_asset_error(format!("failed to write '{}': {error}", absolute.display())))
  }

  fn entries<'a>(&'a self, prefix: Option<&'a str>) -> Box<dyn Iterator<Item = String> + 'a> {
    Box::new(
      self
        .index
        .assets()
        .map(|asset| asset.logical_path().to_string())
        .filter(move |path| prefix.is_none_or(|prefix| is_component_prefix(path, prefix))),
    )
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;

  use xrf_test_utils::utils::get_absolute_generated_test_resource_path;

  use crate::{XrayAssetSource, XrayDirectorySource, XrayMountKind};

  fn source(name: &str, files: &[&str]) -> XrayDirectorySource {
    let root: PathBuf = get_absolute_generated_test_resource_path(&format!("xray_directory_source/{name}"));

    let _ = fs::remove_dir_all(&root);

    for file in files {
      let path: PathBuf = root.join(file);

      fs::create_dir_all(path.parent().expect("file sits in a directory")).expect("test tree is creatable");
      fs::write(&path, b"payload").expect("test file is writable");
    }

    XrayDirectorySource::read(&root).expect("root indexes")
  }

  #[test]
  fn reports_itself_as_a_writable_directory() {
    let source: XrayDirectorySource = source("writable", &["textures/wpn/wpn_ak74.dds"]);

    assert_eq!(source.kind(), XrayMountKind::Directory);
    assert!(source.is_writable());
  }

  #[test]
  fn contains_and_reads_by_logical_path() {
    let source: XrayDirectorySource = source("reads", &["textures/wpn/wpn_ak74.dds"]);

    assert!(source.contains("textures\\wpn\\wpn_ak74.dds"));
    assert!(!source.contains("textures\\wpn\\wpn_val.dds"));
    assert_eq!(source.read("textures\\wpn\\wpn_ak74.dds").unwrap(), b"payload");
  }

  #[test]
  fn refuses_to_read_or_write_a_path_it_does_not_hold() {
    let source: XrayDirectorySource = source("absent", &["textures/wpn/wpn_ak74.dds"]);

    assert!(source.read("textures\\wpn\\wpn_val.dds").is_err());
    assert!(source.write("textures\\wpn\\wpn_val.dds", b"new").is_err());
  }

  #[test]
  fn writes_over_an_entry_it_holds() {
    let source: XrayDirectorySource = source("writes", &["configs\\system.ltx"]);

    source
      .write("configs\\system.ltx", b"formatted")
      .expect("write succeeds");

    assert_eq!(source.read("configs\\system.ltx").unwrap(), b"formatted");
  }

  #[test]
  fn enumerates_every_entry_and_narrows_by_prefix() {
    let source: XrayDirectorySource = source(
      "enumerates",
      &[
        "textures/wpn/wpn_ak74.dds",
        "configs/system.ltx",
        "configs/weapons/ak74.ltx",
      ],
    );

    let all: Vec<String> = source.entries(None).collect();
    let configs: Vec<String> = source.entries(Some("configs")).collect();

    assert_eq!(all.len(), 3);
    assert_eq!(configs.len(), 2);
    assert!(configs.iter().all(|path| path.starts_with("configs\\")));
  }

  #[test]
  fn a_prefix_matches_on_component_boundaries_only() {
    // `configs_backup` must not be swept up by a `configs` prefix, or a scoped operation would touch a sibling tree.
    let source: XrayDirectorySource = source("boundaries", &["configs/system.ltx", "configs_backup/system.ltx"]);

    assert_eq!(source.entries(Some("configs")).count(), 1);
  }
}

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use xrf_error::{XrfError, XrfResult};
use xrf_test_utils::utils::get_absolute_generated_test_resource_path;

use crate::{XrayAssetContainer, XrayAssetSource, XrayMountKind};

/// In-memory archive stand-in that avoids large volume fixtures.
#[derive(Debug)]
pub struct FakeArchiveSource {
  label: String,
  root: PathBuf,
  entries: HashMap<String, Vec<u8>>,
}

impl FakeArchiveSource {
  pub fn new(label: &str, entries: &[&str]) -> Self {
    Self {
      entries: entries
        .iter()
        .map(|path| ((*path).to_string(), label.as_bytes().to_vec()))
        .collect(),
      label: label.to_string(),
      root: PathBuf::from(format!("C:\\install\\db\\{label}")),
    }
  }
}

impl XrayAssetSource for FakeArchiveSource {
  fn label(&self) -> &str {
    &self.label
  }

  fn kind(&self) -> XrayMountKind {
    XrayMountKind::Archive
  }

  fn is_writable(&self) -> bool {
    false
  }

  fn root_path(&self) -> &Path {
    &self.root
  }

  fn contains(&self, path: &str) -> bool {
    self.entries.contains_key(path)
  }

  fn locate(&self, path: &str) -> Option<XrayAssetContainer> {
    self.entries.contains_key(path).then(|| XrayAssetContainer::Archive {
      path: self.root.clone(),
    })
  }

  fn read(&self, path: &str) -> XrfResult<Vec<u8>> {
    self
      .entries
      .get(path)
      .cloned()
      .ok_or_else(|| XrfError::new_asset_error(format!("no entry {path}")))
  }

  fn write(&self, _path: &str, _bytes: &[u8]) -> XrfResult<()> {
    Err(XrfError::new_asset_error("archive is read only"))
  }

  fn create(&self, _path: &str, _bytes: &[u8]) -> XrfResult<()> {
    Err(XrfError::new_asset_error("archive is read only"))
  }

  fn entries<'a>(&'a self, prefix: Option<&'a str>) -> Box<dyn Iterator<Item = String> + 'a> {
    Box::new(
      self
        .entries
        .keys()
        .filter(move |path| prefix.is_none_or(|prefix| path.starts_with(prefix)))
        .cloned(),
    )
  }
}

pub fn directory(name: &str, files: &[&str]) -> PathBuf {
  let root: PathBuf = get_absolute_generated_test_resource_path(&format!("xray_vfs/{name}"));

  let _ = fs::remove_dir_all(&root);

  for file in files {
    let path: PathBuf = root.join(file);

    fs::create_dir_all(path.parent().expect("file sits in a directory")).expect("test tree is creatable");
    fs::write(&path, name.as_bytes()).expect("test file is writable");
  }

  root
}

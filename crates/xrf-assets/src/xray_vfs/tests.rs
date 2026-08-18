use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use xrf_error::{XrfError, XrfResult};
use xrf_test_utils::utils::get_absolute_generated_test_resource_path;

use crate::{XrayAssetContainer, XrayAssetLocation, XrayAssetSource, XrayMountKind, XrayScope, XrayVfs};

/// In-memory archive stand-in that avoids large volume fixtures.
#[derive(Debug)]
struct FakeArchiveSource {
  label: String,
  root: PathBuf,
  entries: HashMap<String, Vec<u8>>,
}

impl FakeArchiveSource {
  fn new(label: &str, entries: &[&str]) -> Self {
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

fn directory(name: &str, files: &[&str]) -> PathBuf {
  let root: PathBuf = get_absolute_generated_test_resource_path(&format!("xray_vfs/{name}"));

  let _ = fs::remove_dir_all(&root);

  for file in files {
    let path: PathBuf = root.join(file);

    fs::create_dir_all(path.parent().expect("file sits in a directory")).expect("test tree is creatable");
    fs::write(&path, name.as_bytes()).expect("test file is writable");
  }

  root
}

#[test]
fn resolves_a_texture_reference_against_a_mounted_root() {
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount_directory("", directory("texture", &["textures/wpn/wpn_ak74.dds"]))
    .expect("root mounts");

  let location: XrayAssetLocation = vfs
    .dds_texture(&XrayScope::all(), "wpn\\wpn_ak74")
    .expect("lookup succeeds")
    .expect("texture resolves");

  assert_eq!(location.logical_path(), "textures\\wpn\\wpn_ak74.dds");
}

#[test]
fn the_first_mount_holding_a_name_wins_and_the_shadowed_copy_stays_visible() {
  // Callers reverse engine declaration order so the first mount is the last-registered winner.
  let overlay: PathBuf = directory("overlay", &["textures/wpn/wpn_ak74.dds"]);
  let base: PathBuf = directory("base", &["textures/wpn/wpn_ak74.dds"]);

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount_directory("", &overlay).expect("overlay mounts");
  vfs.mount_directory("", &base).expect("base mounts");

  let scope: XrayScope = XrayScope::all();

  assert_eq!(
    vfs
      .find(&scope, "textures\\wpn\\wpn_ak74.dds")
      .unwrap()
      .and_then(|it| it.root().map(Path::to_path_buf)),
    Some(overlay)
  );
  assert_eq!(
    vfs.find_all(&scope, "textures\\wpn\\wpn_ak74.dds").unwrap().len(),
    2,
    "the shadowed copy is still reportable"
  );
  assert_eq!(vfs.read(&scope, "textures\\wpn\\wpn_ak74.dds").unwrap(), b"overlay");
}

#[test]
fn a_subtree_mount_carries_engine_identity_through_its_base() {
  // A logical base lets a standalone subtree resolve against full engine paths.
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount_directory("configs\\weapons", directory("subtree", &["ak74.ltx"]))
    .expect("subtree mounts");

  let scope: XrayScope = XrayScope::all();

  assert!(vfs.find(&scope, "configs\\weapons\\ak74.ltx").unwrap().is_some());
  assert!(
    vfs.find(&scope, "ak74.ltx").unwrap().is_none(),
    "a source relative path is not a logical path"
  );
  assert_eq!(
    vfs.entries(&scope).first().map(|it| it.logical_path().to_string()),
    Some(String::from("configs\\weapons\\ak74.ltx"))
  );
}

#[test]
fn an_archived_entry_resolves_and_reads_but_offers_no_physical_path() {
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount(
      "",
      Box::new(FakeArchiveSource::new("textures", &["textures\\wpn\\wpn_ak74.dds"])),
    )
    .expect("archive mounts");

  let scope: XrayScope = XrayScope::all();
  let location: XrayAssetLocation = vfs
    .find(&scope, "textures\\wpn\\wpn_ak74.dds")
    .unwrap()
    .expect("entry resolves");

  assert_eq!(location.physical_path(), None);
  assert_eq!(vfs.read(&scope, "textures\\wpn\\wpn_ak74.dds").unwrap(), b"textures");
}

#[test]
fn a_loose_file_overrides_an_archived_one() {
  // Reversing fsgame registration order puts loose gamedata ahead of archives.
  let loose: PathBuf = directory("loose_wins", &["textures/wpn/wpn_ak74.dds"]);

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount_directory("", &loose).expect("directory mounts");
  vfs
    .mount(
      "",
      Box::new(FakeArchiveSource::new("textures", &["textures\\wpn\\wpn_ak74.dds"])),
    )
    .expect("archive mounts");

  assert_eq!(
    vfs.read(&XrayScope::all(), "textures\\wpn\\wpn_ak74.dds").unwrap(),
    b"loose_wins"
  );
}

#[test]
fn writing_an_archived_winner_is_refused_and_names_the_archive() {
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount(
      "",
      Box::new(FakeArchiveSource::new("configs", &["configs\\system.ltx"])),
    )
    .expect("archive mounts");

  let error: String = vfs
    .write(&XrayScope::all(), "configs\\system.ltx", b"formatted")
    .expect_err("write is refused")
    .to_string();

  assert!(error.contains("read only"), "{error}");
  assert!(error.contains("configs"), "the refusal names what holds it: {error}");
}

#[test]
fn a_writable_scope_skips_an_archive_entirely() {
  // A writable scope lets the same operation skip archives.
  let loose: PathBuf = directory("writable_scope", &["configs/system.ltx"]);

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount("", Box::new(FakeArchiveSource::new("configs", &["configs\\other.ltx"])))
    .expect("archive mounts");
  vfs.mount_directory("", &loose).expect("directory mounts");

  let writable: XrayScope = XrayScope::writable();

  vfs
    .write(&writable, "configs\\system.ltx", b"formatted")
    .expect("a loose winner is writable");

  assert_eq!(vfs.entries(&writable).len(), 1, "only the loose entry is in scope");
  assert!(
    vfs.find(&writable, "configs\\other.ltx").unwrap().is_none(),
    "the archived entry is out of scope"
  );
}

#[test]
fn a_prefix_scope_cannot_answer_outside_its_subtree() {
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount_directory(
      "",
      directory("prefixed", &["configs/system.ltx", "textures/wpn/wpn_ak74.dds"]),
    )
    .expect("root mounts");

  let configs: XrayScope = XrayScope::all().with_prefix("configs").expect("prefix is valid");

  assert!(vfs.find(&configs, "configs\\system.ltx").unwrap().is_some());
  assert!(
    vfs.find(&configs, "textures\\wpn\\wpn_ak74.dds").unwrap().is_none(),
    "a scoped lookup must not reach outside its subtree"
  );
  assert_eq!(vfs.entries(&configs).len(), 1);
}

#[test]
fn mounting_the_same_directory_twice_reuses_the_mount() {
  let root: PathBuf = directory("reused", &["configs/system.ltx"]);

  let mut vfs: XrayVfs = XrayVfs::new();
  let first = vfs.mount_directory("", &root).expect("mounts");
  let second = vfs.mount_directory("", &root).expect("mounts again");

  assert_eq!(first, second);
  assert_eq!(vfs.mount_count(), 1, "the same root is not walked twice");
}

#[test]
fn enumeration_dedupes_across_mounts_and_reports_shadowed_copies_separately() {
  let overlay: PathBuf = directory("dedupe_overlay", &["configs/system.ltx"]);
  let base: PathBuf = directory("dedupe_base", &["configs/system.ltx", "configs/weather.ltx"]);

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount_directory("", &overlay).expect("overlay mounts");
  vfs.mount_directory("", &base).expect("base mounts");

  let scope: XrayScope = XrayScope::all();

  assert_eq!(vfs.entries(&scope).len(), 2, "winners only");
  assert_eq!(vfs.entries_all(&scope).len(), 3, "including the shadowed copy");
}

use std::fs;
use std::path::PathBuf;

use crate::vfs::tests::fake_source::{FakeArchiveSource, directory};
use crate::{XrayAsset, XrayMountId, XrayScope, XrayVfs};

#[test]
fn an_override_creates_a_loose_file_that_then_wins() {
  // A normal write rejects the archived winner; an explicit override creates a loose winner that resolves immediately.
  let loose: PathBuf = directory("override_wins", &["configs/other.ltx"]);

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount_directory("", &loose).expect("directory mounts");
  vfs
    .mount(
      "",
      Box::new(FakeArchiveSource::new("configs", &["configs\\system.ltx"])),
    )
    .expect("archive mounts");

  let scope: XrayScope = XrayScope::all();

  assert!(
    vfs.write(&scope, "configs\\system.ltx", b"formatted").is_err(),
    "the archived winner refuses a plain write"
  );

  let location: XrayAsset = vfs
    .write_override(&scope, "configs\\system.ltx", b"overridden")
    .expect("override is created");

  assert_eq!(location.root(), Some(loose.as_path()), "it lands in the writable mount");
  assert_eq!(
    vfs.read(&scope, "configs\\system.ltx").unwrap(),
    b"overridden",
    "and wins immediately, without the caller remounting"
  );
  assert_eq!(
    vfs.find_all(&scope, "configs\\system.ltx").unwrap().len(),
    2,
    "the archived copy is still reportable behind it"
  );
}

#[test]
fn an_override_is_refused_when_no_writable_mount_is_in_scope() {
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount(
      "",
      Box::new(FakeArchiveSource::new("configs", &["configs\\system.ltx"])),
    )
    .expect("archive mounts");

  let error: String = vfs
    .write_override(&XrayScope::all(), "configs\\system.ltx", b"overridden")
    .expect_err("override is refused")
    .to_string();

  assert!(error.contains("no writable mount"), "{error}");
}

#[test]
fn an_override_refuses_to_replace_a_file_the_writable_mount_already_holds() {
  // That is an overwrite, which `write` already does. Accepting it here would make the two indistinguishable.
  let loose: PathBuf = directory("override_existing", &["configs/system.ltx"]);

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount_directory("", &loose).expect("directory mounts");

  assert!(
    vfs
      .write_override(&XrayScope::all(), "configs\\system.ltx", b"overridden")
      .is_err()
  );
}

#[test]
fn an_override_outside_the_writable_mount_base_is_refused() {
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount_directory("configs\\weapons", directory("override_base", &["ak74.ltx"]))
    .expect("subtree mounts");

  let error: String = vfs
    .write_override(&XrayScope::all(), "textures\\wpn\\wpn_ak74.dds", b"bytes")
    .expect_err("override is refused")
    .to_string();

  assert!(error.contains("base"), "{error}");
}

#[test]
fn remounting_a_directory_picks_up_a_file_written_behind_the_vfs() {
  // Mount-time indexes miss external writes; remounting refreshes them, as `write_override` does internally.
  let root: PathBuf = directory("remount", &["configs/system.ltx"]);

  let mut vfs: XrayVfs = XrayVfs::new();
  let id: XrayMountId = vfs.mount_directory("", &root).expect("directory mounts");

  fs::write(root.join("configs/weather.ltx"), b"weather").expect("file written outside the vfs");

  assert!(vfs.find(&XrayScope::all(), "configs\\weather.ltx").unwrap().is_none());

  vfs.remount(id).expect("directory remounts");

  assert!(vfs.find(&XrayScope::all(), "configs\\weather.ltx").unwrap().is_some());
}

#[test]
fn an_override_lands_in_the_highest_priority_writable_mount() {
  // With two loose mounts the override must shadow both, which means the first one in order.
  let front: PathBuf = directory("override_front", &["configs/other.ltx"]);
  let back: PathBuf = directory("override_back", &["configs/system.ltx"]);

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount_directory("", &front).expect("front mounts");
  vfs.mount_directory("", &back).expect("back mounts");

  let location: XrayAsset = vfs
    .write_override(&XrayScope::all(), "configs\\system.ltx", b"overridden")
    .expect("override is created");

  assert_eq!(location.root(), Some(front.as_path()));
  assert_eq!(
    vfs.read(&XrayScope::all(), "configs\\system.ltx").unwrap(),
    b"overridden"
  );
}

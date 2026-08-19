//! Mounts a freshly packed volume set as an asset source.
//!
//! Packing and reading back is the only honest check: an archive source is correct when it answers for volumes the packer
//! actually wrote, in the name form the header actually stores.

use std::fs;
use std::path::PathBuf;

use xrf_test_utils::utils::get_absolute_generated_test_resource_path;
use xrf_vfs::ArchiveAssetSource;
use xrf_vfs::{XrayAssetContainer, XrayAssetSource, XrayMountKind, XrayScope, XrayVfs};

use crate::pack::archive_pack_config::{ArchivePackConfig, ArchivePackFolder};
use crate::pack::archive_packer::ArchivePacker;

const TEXTURE: &[u8] = &[0x44, 0x44, 0x53, 0x20, 0x01, 0x02, 0x03, 0xfe];
const CONFIG: &[u8] = b"[section]\nvalue = 1\n";

/// Packs a source tree into volumes and mounts the result.
fn mount(scope: &str, files: &[(&str, &[u8])]) -> ArchiveAssetSource {
  let source: PathBuf = get_absolute_generated_test_resource_path(&format!("archive_asset_source/{scope}/gamedata"));
  let destination: PathBuf = get_absolute_generated_test_resource_path(&format!("archive_asset_source/{scope}/db"));

  let _ = fs::remove_dir_all(&source);
  let _ = fs::remove_dir_all(&destination);

  for (name, contents) in files {
    let path: PathBuf = source.join(name.replace('\\', "/"));

    fs::create_dir_all(path.parent().expect("entry parent")).expect("source directory");
    fs::write(&path, contents).expect("source file");
  }

  let mut config: ArchivePackConfig = ArchivePackConfig::new(&source, &destination, "packed");

  config.include_folders = vec![ArchivePackFolder {
    is_recursive: true,
    path: String::new(),
  }];

  ArchivePacker::pack(&config).expect("archive packs");

  ArchiveAssetSource::read(&destination).expect("volume set mounts")
}

#[test]
fn reports_itself_as_a_read_only_archive() {
  let source: ArchiveAssetSource = mount("read_only", &[("textures\\wpn\\wpn_ak74.dds", TEXTURE)]);

  assert_eq!(source.kind(), XrayMountKind::Archive);
  assert!(!source.is_writable());
  assert!(source.write("textures\\wpn\\wpn_ak74.dds", TEXTURE).is_err());
}

#[test]
fn contains_and_reads_a_packed_entry_by_logical_path() {
  let source: ArchiveAssetSource = mount("reads", &[("textures\\wpn\\wpn_ak74.dds", TEXTURE)]);

  assert!(source.contains("textures\\wpn\\wpn_ak74.dds"));
  assert!(!source.contains("textures\\wpn\\wpn_val.dds"));
  assert_eq!(source.read("textures\\wpn\\wpn_ak74.dds").unwrap(), TEXTURE);
}

#[test]
fn locates_an_entry_in_its_volume_set_rather_than_on_disk() {
  // The container is the whole reason an archived asset cannot be handed to `fs::read`.
  let source: ArchiveAssetSource = mount("locates", &[("textures\\wpn\\wpn_ak74.dds", TEXTURE)]);

  assert!(matches!(
    source.locate("textures\\wpn\\wpn_ak74.dds"),
    Some(XrayAssetContainer::Archive { .. })
  ));
  assert_eq!(source.locate("textures\\wpn\\wpn_val.dds"), None);
}

#[test]
fn enumerates_entries_and_narrows_by_prefix() {
  let source: ArchiveAssetSource = mount(
    "enumerates",
    &[
      ("textures\\wpn\\wpn_ak74.dds", TEXTURE),
      ("configs\\system.ltx", CONFIG),
      ("configs\\weapons\\ak74.ltx", CONFIG),
    ],
  );

  // Only files. A volume also records the directories it contains, and those must not surface as assets.
  assert_eq!(source.entries(None).count(), 3);
  assert_eq!(source.entries(Some("configs")).count(), 2);
  assert!(!source.contains("configs"), "a directory entry is not an asset");
  assert!(!source.contains("textures\\wpn"));
}

#[test]
fn resolves_a_texture_reference_once_mounted_in_a_vfs() {
  // What the visuals viewer will do against a real install: the reference completes to a logical path and the archive
  // answers it, with no filesystem path anywhere in the chain.
  let source: ArchiveAssetSource = mount("vfs", &[("textures\\wpn\\wpn_ak74.dds", TEXTURE)]);

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount("", Box::new(source)).expect("archive mounts");

  let scope: XrayScope = XrayScope::all();
  let location = vfs
    .dds_texture(&scope, "wpn\\wpn_ak74")
    .expect("lookup succeeds")
    .expect("texture resolves");

  assert_eq!(location.logical_path(), "textures\\wpn\\wpn_ak74.dds");
  assert_eq!(location.physical_path(), None);
  assert_eq!(vfs.read(&scope, "textures\\wpn\\wpn_ak74.dds").unwrap(), TEXTURE);
}

#[test]
fn a_loose_file_wins_over_the_same_name_in_an_archive() {
  // The rule fsgame declares by listing db before gamedata. Mount order carries it.
  let archived: ArchiveAssetSource = mount("override", &[("textures\\wpn\\wpn_ak74.dds", TEXTURE)]);

  let loose: PathBuf = get_absolute_generated_test_resource_path("archive_asset_source/override/loose");

  let _ = fs::remove_dir_all(&loose);

  fs::create_dir_all(loose.join("textures/wpn")).expect("loose directory");
  fs::write(loose.join("textures/wpn/wpn_ak74.dds"), b"loose").expect("loose file");

  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount_directory("", &loose).expect("directory mounts");
  vfs.mount("", Box::new(archived)).expect("archive mounts");

  let scope: XrayScope = XrayScope::all();

  assert_eq!(vfs.read(&scope, "textures\\wpn\\wpn_ak74.dds").unwrap(), b"loose");
  assert_eq!(
    vfs.find_all(&scope, "textures\\wpn\\wpn_ak74.dds").unwrap().len(),
    2,
    "the archived copy stays reportable behind the override"
  );
}

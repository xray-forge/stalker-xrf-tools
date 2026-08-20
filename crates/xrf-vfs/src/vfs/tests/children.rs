//! Lists what sits directly inside a logical directory.
//!
//! This is what a browser or tree view expands one node with, and what a wildcard include means by one directory. It is
//! deliberately not `entries` with a prefix, which answers everything below.

use crate::vfs::tests::fake_source::{FakeArchiveSource, directory};
use crate::{XrayDirectoryListing, XrayScope, XrayVfs};

fn mounted(name: &str, files: &[&str]) -> XrayVfs {
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs.mount_directory("", directory(name, files)).expect("root mounts");

  vfs
}

#[test]
fn separates_folders_from_files_directly_inside() {
  let vfs: XrayVfs = mounted(
    "children_split",
    &[
      "configs/system.ltx",
      "configs/weapons/w_ak74.ltx",
      "configs/weapons/w_pm.ltx",
      "configs/environment/weathers/default.ltx",
    ],
  );

  let listing: XrayDirectoryListing = vfs.children(&XrayScope::all(), "configs").expect("listing");

  assert_eq!(listing.directories, vec!["environment", "weapons"]);
  assert_eq!(
    listing
      .files
      .iter()
      .map(|file| file.logical_path().to_string())
      .collect::<Vec<_>>(),
    vec!["configs\\system.ltx"],
    "only the file sitting directly inside is listed"
  );
}

#[test]
fn does_not_answer_with_everything_below_the_directory() {
  // The difference from `entries` with a prefix scope, which is what a tree view must not do.
  let vfs: XrayVfs = mounted(
    "children_shallow",
    &["textures/wpn/wpn_ak74.dds", "textures/wpn/scopes/scope.dds"],
  );

  let listing: XrayDirectoryListing = vfs.children(&XrayScope::all(), "textures").expect("listing");

  assert_eq!(listing.directories, vec!["wpn"]);
  assert!(listing.files.is_empty());
  assert_eq!(
    vfs
      .entries(&XrayScope::all().with_prefix("textures").expect("prefix"))
      .len(),
    2,
    "entries still answers the whole subtree"
  );
}

#[test]
fn lists_the_logical_root_for_an_empty_directory() {
  let vfs: XrayVfs = mounted("children_root", &["configs/system.ltx", "textures/wpn/wpn_ak74.dds"]);

  let listing: XrayDirectoryListing = vfs.children(&XrayScope::all(), "").expect("listing");

  assert_eq!(listing.directories, vec!["configs", "textures"]);
  assert!(listing.files.is_empty());
}

#[test]
fn merges_children_across_mounts_and_dedupes_folders() {
  let mut vfs: XrayVfs = mounted("children_overlay", &["configs/weapons/w_ak74.ltx"]);

  vfs
    .mount_directory(
      "",
      directory("children_base", &["configs/weapons/w_pm.ltx", "configs/system.ltx"]),
    )
    .expect("base mounts");

  let listing: XrayDirectoryListing = vfs.children(&XrayScope::all(), "configs").expect("listing");

  assert_eq!(listing.directories, vec!["weapons"], "one folder, not one per mount");
  assert_eq!(listing.files.len(), 1);
}

#[test]
fn sees_archived_children_beside_loose_ones() {
  let mut vfs: XrayVfs = XrayVfs::new();

  vfs
    .mount_directory("", directory("children_loose", &["configs/system.ltx"]))
    .expect("loose mounts");
  vfs
    .mount(
      "",
      Box::new(FakeArchiveSource::new(
        "children_archive",
        &["configs/weapons/w_ak74.ltx"],
      )),
    )
    .expect("archive mounts");

  let listing: XrayDirectoryListing = vfs.children(&XrayScope::all(), "configs").expect("listing");

  assert_eq!(listing.directories, vec!["weapons"]);
  assert_eq!(listing.files.len(), 1);
}

#[test]
fn answers_an_empty_listing_for_a_directory_nothing_holds() {
  let vfs: XrayVfs = mounted("children_absent", &["configs/system.ltx"]);

  assert!(vfs.children(&XrayScope::all(), "meshes").expect("listing").is_empty());
}

#[test]
fn rejects_a_directory_that_is_not_a_logical_path() {
  let vfs: XrayVfs = mounted("children_invalid", &["configs/system.ltx"]);

  assert!(vfs.children(&XrayScope::all(), "configs\\..\\textures").is_err());
}

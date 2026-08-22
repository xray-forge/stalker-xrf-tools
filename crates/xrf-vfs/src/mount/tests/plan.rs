use std::fs;
use std::path::{Path, PathBuf};

use xrf_test_utils::utils::build_absolute_generated_test_resource_path;

use crate::{FsgameFile, XrayMountPlan, XraySourceKind};

/// The alias order a real `fsgame.ltx` uses: archives declared before gamedata, which is what makes loose files win.
const FSGAME: &str = "\
;abbreviation       = recurs| notif | root        | add
$app_data_root$     = true  | false | $fs_root$   | appdata\\
$arch_dir$          = false | false | $fs_root$   | db\\
$arch_dir_textures$ = false | false | $arch_dir$  | textures\\
$game_data$         = true  | true  | $fs_root$   | gamedata\\
$game_meshes$       = true  | true  | $game_data$ | meshes\\
";

/// Builds an installation on disk, since a plan is decided by what is actually there.
///
/// `files` are paths relative to the installation, so a caller writes `db\\textures\\textures.db0` to place a volume.
fn install(name: &str, files: &[&str]) -> PathBuf {
  let root: PathBuf = build_absolute_generated_test_resource_path(&format!("xray_mount_plan/{name}"));

  let _ = fs::remove_dir_all(&root);

  fs::create_dir_all(&root).expect("install root");
  fs::write(root.join("fsgame.ltx"), FSGAME).expect("fsgame written");

  for file in files {
    let path: PathBuf = root.join(file.replace('\\', "/"));

    fs::create_dir_all(path.parent().expect("entry parent")).expect("install directory");
    fs::write(&path, b"payload").expect("install file");
  }

  root
}

fn plan(name: &str, files: &[&str]) -> XrayMountPlan {
  let root: PathBuf = install(name, files);

  XrayMountPlan::from_fsgame(&root).expect("fsgame plans")
}

#[test]
fn plans_gamedata_ahead_of_archives() {
  // The gate for this whole layer: fsgame declares db before gamedata, so the plan must reverse that. Getting it
  // backwards would make every archived file shadow the loose one that is supposed to override it.
  let plan: XrayMountPlan = plan(
    "ordering",
    &["db\\textures\\textures.db0", "gamedata\\textures\\wpn\\wpn_ak74.dds"],
  );

  let origins: Vec<&str> = plan.get_mounts().iter().map(|it| it.origin.as_str()).collect();

  assert_eq!(origins, vec!["$game_data$", "$arch_dir_textures$"]);
  assert_eq!(plan.get_mounts()[0].kind, XraySourceKind::Directory);
  assert_eq!(plan.get_mounts()[1].kind, XraySourceKind::Archive);
}

#[test]
fn plans_a_directory_of_volumes_as_an_archive() {
  let plan: XrayMountPlan = plan("volumes", &["db\\textures\\textures.db0", "db\\textures\\textures.db1"]);

  assert_eq!(plan.len(), 1);
  assert_eq!(plan.get_mounts()[0].kind, XraySourceKind::Archive);
  assert!(plan.get_mounts()[0].path.ends_with("textures"));
}

#[test]
fn leaves_output_directories_out_of_the_plan() {
  // appdata holds saves and screenshots. Mounting it as an asset source would put them in the same table as textures.
  let plan: XrayMountPlan = plan(
    "outputs",
    &["appdata\\savedgames\\quicksave.sav", "gamedata\\configs\\system.ltx"],
  );

  let origins: Vec<&str> = plan.get_mounts().iter().map(|it| it.origin.as_str()).collect();

  assert_eq!(origins, vec!["$game_data$"]);
}

#[test]
fn plans_gamedata_once_rather_than_once_per_subdirectory_alias() {
  // `$game_meshes$` resolves inside gamedata. Mounting both would register every mesh twice, as `meshes\x.ogf` and
  // again as `x.ogf`.
  let plan: XrayMountPlan = plan("subdirectories", &["gamedata\\meshes\\wpn.ogf"]);

  assert_eq!(plan.len(), 1);
  assert!(plan.get_mounts()[0].path.ends_with("gamedata"));
}

#[test]
fn plans_an_empty_gamedata_because_it_is_still_where_an_override_would_go() {
  // Anomaly ships exactly this: everything in volumes and an empty gamedata. Treating the emptiness as "not a source"
  // left a writable scope with no mounts at all, so an override had nowhere to land.
  let root: PathBuf = install("empty_gamedata", &["db\\textures\\textures.db0"]);

  fs::create_dir_all(root.join("gamedata")).expect("empty gamedata directory");

  let plan: XrayMountPlan = XrayMountPlan::from_fsgame(&root).expect("fsgame plans");

  let origins: Vec<&str> = plan.get_mounts().iter().map(|it| it.origin.as_str()).collect();

  assert_eq!(origins, vec!["$game_data$", "$arch_dir_textures$"]);
  assert_eq!(plan.get_mounts()[0].kind, XraySourceKind::Directory);
}

#[test]
fn skips_a_declared_directory_that_is_not_there() {
  let plan: XrayMountPlan = plan("absent", &["gamedata\\configs\\system.ltx"]);

  assert_eq!(plan.len(), 1, "db was never created, so nothing plans for it");
}

#[test]
fn plans_a_bare_root_without_any_fsgame() {
  let plan: XrayMountPlan = XrayMountPlan::root("C:\\gamedata").expect("root plans");

  assert_eq!(plan.len(), 1);
  assert_eq!(plan.get_mounts()[0].base, "");
  assert_eq!(plan.get_mounts()[0].kind, XraySourceKind::Directory);
  assert_eq!(plan.get_mounts()[0].origin, "root");
}

#[test]
fn plans_a_subtree_at_its_logical_base() {
  let plan: XrayMountPlan = XrayMountPlan::subtree("C:\\loose\\weapons", "Configs/Weapons").expect("subtree plans");

  assert_eq!(plan.get_mounts()[0].base, "configs\\weapons", "the base is normalized");
}

#[test]
fn plans_the_root_implied_by_an_asset_and_nothing_when_there_is_none() {
  let root: PathBuf = build_absolute_generated_test_resource_path("xray_mount_plan/implied");

  let _ = fs::remove_dir_all(&root);

  fs::create_dir_all(root.join("meshes/dynamics")).expect("meshes directory");
  fs::create_dir_all(root.join("textures")).expect("textures directory");

  let implied: XrayMountPlan = XrayMountPlan::implied(root.join("meshes/dynamics/wpn.ogf")).expect("implied plans");

  assert_eq!(implied.get_mounts()[0].path, root);
  assert!(
    XrayMountPlan::implied(Path::new("C:\\nowhere\\wpn.ogf"))
      .expect("implied plans")
      .is_empty(),
    "a guess that finds nothing plans nothing rather than guessing anyway"
  );
}

#[test]
fn chains_a_fallback_behind_a_plan_without_repeating_a_path() {
  // How the viewer expresses "the tree the model came from, then the configured project".
  let chained: XrayMountPlan = XrayMountPlan::root("C:\\first")
    .expect("root plans")
    .behind(XrayMountPlan::root("C:\\second").expect("root plans"))
    .behind(XrayMountPlan::root("C:\\first").expect("root plans"));

  let paths: Vec<&Path> = chained.get_mounts().iter().map(|it| it.path.as_path()).collect();

  assert_eq!(paths, vec![Path::new("C:\\first"), Path::new("C:\\second")]);
}

#[test]
fn reports_an_installation_with_no_fsgame_as_an_error_rather_than_an_empty_plan() {
  let root: PathBuf = build_absolute_generated_test_resource_path("xray_mount_plan/no_fsgame");

  let _ = fs::remove_dir_all(&root);

  fs::create_dir_all(&root).expect("root");

  assert!(XrayMountPlan::from_fsgame(&root).is_err());
}

#[test]
fn plans_from_an_already_parsed_fsgame() {
  let root: PathBuf = install("parsed", &["gamedata\\configs\\system.ltx"]);
  let fsgame: FsgameFile = FsgameFile::read(&root).expect("fsgame reads");

  assert_eq!(XrayMountPlan::from_fsgame_file(&fsgame).expect("plans").len(), 1);
}

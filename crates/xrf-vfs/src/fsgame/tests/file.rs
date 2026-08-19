use std::path::{Path, PathBuf};

use crate::fsgame::file::FsgameFile;

/// The shape a real `fsgame.ltx` has, trimmed to the aliases these tests reason about.
const CONTENTS: &str = "\
;abbreviation           = recurs| notif | root           | add        | ext         | description
$app_data_root$         = true  | false | $fs_root$      | appdata\\
$arch_dir$              = false | false | $fs_root$      | db\\
$arch_dir_textures$     = false | false | $arch_dir$     | textures\\
$game_data$             = true  | true  | $fs_root$      | gamedata\\
$game_meshes$           = true  | true  | $game_data$    | meshes\\     | *.ogf;*.omf | Game Object files
$downloads$             = false | false | $app_data_root$
";

fn file() -> FsgameFile {
  FsgameFile::parse(Path::new("C:\\install"), CONTENTS).expect("contents parse")
}

#[test]
fn keeps_declarations_in_the_order_the_file_lists_them() {
  // Order is the override rule: `$arch_dir$` before `$game_data$` is what makes loose files win.
  let file: FsgameFile = file();
  let aliases: Vec<&str> = file.declarations().iter().map(|it| it.alias.as_str()).collect();

  assert_eq!(
    aliases,
    vec![
      "$app_data_root$",
      "$arch_dir$",
      "$arch_dir_textures$",
      "$game_data$",
      "$game_meshes$",
      "$downloads$"
    ]
  );
}

#[test]
fn resolves_an_alias_against_the_installation_root() {
  assert_eq!(
    file().resolve("$game_data$"),
    Some(PathBuf::from("C:\\install\\gamedata"))
  );
}

#[test]
fn resolves_a_chain_of_aliases() {
  assert_eq!(
    file().resolve("$arch_dir_textures$"),
    Some(PathBuf::from("C:\\install\\db\\textures"))
  );
  assert_eq!(
    file().resolve("$game_meshes$"),
    Some(PathBuf::from("C:\\install\\gamedata\\meshes"))
  );
}

#[test]
fn an_alias_that_only_names_its_root_resolves_to_that_root() {
  assert_eq!(
    file().resolve("$downloads$"),
    Some(PathBuf::from("C:\\install\\appdata"))
  );
}

#[test]
fn answers_none_for_an_undeclared_alias() {
  assert_eq!(file().resolve("$nonexistent$"), None);
}

#[test]
fn a_cycle_terminates_instead_of_recursing() {
  let file: FsgameFile = FsgameFile::parse(
    Path::new("C:\\install"),
    "$first$ = false | false | $second$ | a\\\n$second$ = false | false | $first$ | b\\\n",
  )
  .expect("contents parse");

  assert_eq!(file.resolve("$first$"), None);
}

#[test]
fn resolves_every_declared_alias_at_once() {
  let resolved = file().resolve_paths();

  assert_eq!(resolved.len(), 6);
  assert_eq!(resolved.get("$arch_dir$"), Some(&PathBuf::from("C:\\install\\db")));
}

#[test]
fn rejects_contents_declaring_nothing() {
  assert!(FsgameFile::parse(Path::new("C:\\install"), ";only a comment\n").is_err());
}

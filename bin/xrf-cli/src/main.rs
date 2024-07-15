mod commands;

use crate::commands::build_translations::build_translations;
use crate::commands::format_ltx::format_ltx;
use crate::commands::info_spawn_file::info_spawn_file;
use crate::commands::pack_equipment_icons::pack_equipment_icons;
use crate::commands::pack_spawn_file::pack_spawn_file;
use crate::commands::pack_texture_description::pack_texture_description;
use crate::commands::parse_translations::parse_translations;
use crate::commands::repack_spawn_file::repack_spawn_file;
use crate::commands::setup::setup_commands;
use crate::commands::unpack_archive::unpack_archive;
use crate::commands::unpack_equipment_icons::unpack_equipment_icons;
use crate::commands::unpack_spawn_file::unpack_spawn_file;
use crate::commands::unpack_texture_description::unpack_texture_description;
use crate::commands::verify_ltx::verify_ltx;
use crate::commands::verify_spawn_file::verify_spawn_file;
use crate::commands::verify_translations::verify_translations;
use std::env;

#[tokio::main]
async fn main() {
  setup_logger();

  match setup_commands().get_matches().subcommand() {
    Some(("build-translations", matches)) => build_translations(matches),
    Some(("parse-translations", matches)) => parse_translations(matches),
    Some(("verify-translations", matches)) => verify_translations(matches),
    Some(("format-ltx", matches)) => format_ltx(matches),
    Some(("info-spawn", matches)) => info_spawn_file(matches),
    Some(("pack-equipment-icons", matches)) => pack_equipment_icons(matches),
    Some(("pack-spawn", matches)) => pack_spawn_file(matches).unwrap(),
    Some(("pack-texture-description", matches)) => pack_texture_description(matches),
    Some(("repack-spawn", matches)) => repack_spawn_file(matches).unwrap(),
    Some(("unpack-archive", matches)) => unpack_archive(matches).await,
    Some(("unpack-equipment-icons", matches)) => unpack_equipment_icons(matches),
    Some(("unpack-spawn", matches)) => unpack_spawn_file(matches).unwrap(),
    Some(("unpack-texture-description", matches)) => unpack_texture_description(matches),
    Some(("verify-ltx", matches)) => verify_ltx(matches),
    Some(("verify-spawn", matches)) => verify_spawn_file(matches),
    _ => panic!("Unexpected cli command provided, check --help for details"),
  };
}

/// Configure environment logger, fallback to info level.
pub fn setup_logger() {
  if env::var("RUST_LOG").is_err() {
    env::set_var(
      "RUST_LOG",
      match cfg!(debug_assertions) {
        true => "info",
        false => "error",
      },
    )
  }

  env_logger::init();
}

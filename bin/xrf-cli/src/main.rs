mod commands;

use crate::commands::build_translations::{build_translations, create_build_translations_command};
use crate::commands::format_ltx::{create_format_ltx_command, format_ltx};
use crate::commands::info_spawn_file::{create_info_spawn_file_command, info_spawn_file};
use crate::commands::pack_equipment_icons::{
  create_pack_equipment_icons_command, pack_equipment_icons,
};
use crate::commands::pack_spawn_file::{create_pack_spawn_file_command, pack_spawn_file};
use crate::commands::pack_texture_description::{
  create_pack_texture_description_command, pack_texture_description,
};
use crate::commands::parse_translations::{create_parse_translations_command, parse_translations};
use crate::commands::repack_spawn_file::{create_repack_spawn_file_command, repack_spawn_file};
use crate::commands::unpack_archive::{create_unpack_archive_command, unpack_archive};
use crate::commands::unpack_equipment_icons::{
  create_unpack_equipment_icons_command, unpack_equipment_icons,
};
use crate::commands::unpack_spawn_file::{create_unpack_spawn_file_command, unpack_spawn_file};
use crate::commands::unpack_texture_description::{
  create_unpack_texture_description_command, unpack_texture_description,
};
use crate::commands::verify_ltx::{create_verify_ltx_command, verify_ltx};
use crate::commands::verify_spawn_file::{create_verify_spawn_file_command, verify_spawn_file};
use crate::commands::verify_translations::{
  create_verify_translations_command, verify_translations,
};
use clap::Command;
use std::env;

#[tokio::main]
async fn main() {
  setup_logger();
  let command: Command = Command::new("xrf-tool")
    .about("XRF forge CLI tools application")
    .subcommand(create_build_translations_command())
    .subcommand(create_format_ltx_command())
    .subcommand(create_info_spawn_file_command())
    .subcommand(create_pack_equipment_icons_command())
    .subcommand(create_pack_spawn_file_command())
    .subcommand(create_pack_texture_description_command())
    .subcommand(create_parse_translations_command())
    .subcommand(create_repack_spawn_file_command())
    .subcommand(create_unpack_archive_command())
    .subcommand(create_unpack_equipment_icons_command())
    .subcommand(create_unpack_spawn_file_command())
    .subcommand(create_unpack_texture_description_command())
    .subcommand(create_verify_ltx_command())
    .subcommand(create_verify_spawn_file_command())
    .subcommand(create_verify_translations_command());

  match command.get_matches().subcommand() {
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

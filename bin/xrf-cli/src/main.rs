mod commands;

use crate::commands::build_translations::BuildTranslationsCommand;
use crate::commands::format_ltx::FormatLtxCommand;
use crate::commands::info_spawn::InfoSpawnCommand;
use crate::commands::initialize_translations::InitializeTranslationsCommand;
use crate::commands::pack_equipment_icons::PackEquipmentIconsCommand;
use crate::commands::pack_spawn::PackSpawnFileCommand;
use crate::commands::pack_texture_description::PackTextureDescriptionCommand;
use crate::commands::parse_translations::ParseTranslationsCommand;
use crate::commands::repack_spawn::RepackSpawnCommand;
use crate::commands::unpack_archive::UnpackArchiveCommand;
use crate::commands::unpack_equipment_icons::UnpackEquipmentIconsCommand;
use crate::commands::unpack_spawn::UnpackSpawnFileCommand;
use crate::commands::unpack_texture_description::UnpackTextureDescriptionCommand;
use crate::commands::verify_ltx::VerifyLtxCommand;
use crate::commands::verify_spawn::VerifySpawnFileCommand;
use crate::commands::verify_translations::VerifyTranslationsCommand;

use clap::Command;
use std::env;

#[tokio::main]
async fn main() {
  setup_logger();

  let command: Command = Command::new("xrf-tool")
    .about("XRF forge CLI tools application")
    .subcommand(BuildTranslationsCommand::init())
    .subcommand(FormatLtxCommand::init())
    .subcommand(InfoSpawnCommand::init())
    .subcommand(InitializeTranslationsCommand::init())
    .subcommand(PackEquipmentIconsCommand::init())
    .subcommand(PackSpawnFileCommand::init())
    .subcommand(PackTextureDescriptionCommand::init())
    .subcommand(ParseTranslationsCommand::init())
    .subcommand(RepackSpawnCommand::init())
    .subcommand(UnpackArchiveCommand::init())
    .subcommand(UnpackEquipmentIconsCommand::init())
    .subcommand(UnpackSpawnFileCommand::init())
    .subcommand(UnpackTextureDescriptionCommand::init())
    .subcommand(VerifyLtxCommand::init())
    .subcommand(VerifySpawnFileCommand::init())
    .subcommand(VerifyTranslationsCommand::init());

  match command.get_matches().subcommand() {
    Some((BuildTranslationsCommand::NAME, matches)) => {
      BuildTranslationsCommand::execute(matches).unwrap()
    }
    Some((FormatLtxCommand::NAME, matches)) => FormatLtxCommand::execute(matches),
    Some((InfoSpawnCommand::NAME, matches)) => InfoSpawnCommand::execute(matches),
    Some((InitializeTranslationsCommand::NAME, matches)) => {
      InitializeTranslationsCommand::execute(matches).unwrap()
    }
    Some((PackEquipmentIconsCommand::NAME, matches)) => PackEquipmentIconsCommand::execute(matches),
    Some((PackSpawnFileCommand::NAME, matches)) => PackSpawnFileCommand::execute(matches).unwrap(),
    Some((PackTextureDescriptionCommand::NAME, matches)) => {
      PackTextureDescriptionCommand::execute(matches)
    }
    Some((ParseTranslationsCommand::NAME, matches)) => ParseTranslationsCommand::execute(matches),
    Some((RepackSpawnCommand::NAME, matches)) => RepackSpawnCommand::execute(matches).unwrap(),
    Some((UnpackArchiveCommand::NAME, matches)) => UnpackArchiveCommand::execute(matches).await,
    Some((UnpackEquipmentIconsCommand::NAME, matches)) => {
      UnpackEquipmentIconsCommand::execute(matches)
    }
    Some((UnpackSpawnFileCommand::NAME, matches)) => {
      UnpackSpawnFileCommand::execute(matches).unwrap()
    }
    Some((UnpackTextureDescriptionCommand::NAME, matches)) => {
      UnpackTextureDescriptionCommand::execute(matches)
    }
    Some((VerifyLtxCommand::NAME, matches)) => VerifyLtxCommand::execute(matches),
    Some((VerifySpawnFileCommand::NAME, matches)) => VerifySpawnFileCommand::execute(matches),
    Some((VerifyTranslationsCommand::NAME, matches)) => {
      VerifyTranslationsCommand::execute(matches).unwrap()
    }
    _ => panic!("Unexpected cli command provided, check --help for details"),
  };
}

/// Configure environment logger, fallback to info level.
pub fn setup_logger() {
  unsafe {
    if env::var("RUST_LOG").is_err() {
      env::set_var(
        "RUST_LOG",
        match cfg!(debug_assertions) {
          true => "info",
          false => "error",
        },
      )
    }
  }

  env_logger::init();
}

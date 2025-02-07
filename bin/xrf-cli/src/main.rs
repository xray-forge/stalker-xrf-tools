pub(crate) mod commands;
pub(crate) mod generic_command;

use commands::archive::unpack_archive::UnpackArchiveCommand;
use commands::ltx::verify_ltx::VerifyLtxCommand;
use commands::spawn::info_spawn::InfoSpawnCommand;
use commands::spawn::pack_spawn::PackSpawnFileCommand;
use commands::spawn::repack_spawn::RepackSpawnCommand;
use commands::spawn::unpack_spawn::UnpackSpawnFileCommand;
use commands::spawn::verify_spawn::VerifySpawnFileCommand;
use commands::texture::pack_equipment_icons::PackEquipmentIconsCommand;
use commands::texture::pack_texture_description::PackTextureDescriptionCommand;
use commands::texture::unpack_equipment_icons::UnpackEquipmentIconsCommand;
use commands::texture::unpack_texture_description::UnpackTextureDescriptionCommand;
use commands::translation::build_translations::BuildTranslationsCommand;
use commands::translation::initialize_translations::InitializeTranslationsCommand;
use commands::translation::parse_translations::ParseTranslationsCommand;
use commands::translation::verify_translations::VerifyTranslationsCommand;

use crate::commands::gamedata::verify_gamedata::VerifyGamedataCommand;
use crate::commands::ltx::format_ltx::FormatLtxCommand;
use crate::commands::texture::info_dds::InfoDdsCommand;
use crate::generic_command::{CommandResult, GenericCommand};
use clap::Command;
use commands::ogf::info_ogf::InfoOgfCommand;
use commands::omf::info_omf::InfoOmfCommand;
use commands::particle::info_particles::InfoParticlesCommand;
use commands::particle::pack_particles::PackParticlesFileCommand;
use commands::particle::repack_particles::RepackParticlesCommand;
use commands::particle::reunpack_particles::ReUnpackParticlesCommand;
use commands::particle::unpack_particles::UnpackParticlesCommand;
use commands::particle::verify_particles::VerifyParticlesFileCommand;
use env_logger::Builder;
use log::LevelFilter;
use std::error::Error;
use std::{env, process};

fn main() -> Result<(), Box<dyn Error>> {
  setup_logger();

  let mut command: Command = Command::new("xrf-tool").about("XRF forge CLI tools application");
  let subcommands: Vec<Box<dyn GenericCommand>> = setup_subcommands();

  for subcommand in &subcommands {
    command = command.subcommand(subcommand.init());
  }

  if let Some((command_name, matches)) = command.get_matches().subcommand() {
    subcommands
      .iter()
      .find(|it| it.name() == command_name)
      .map(|it| {
        let result: CommandResult = it.execute(matches);

        if let Err(error) = &result {
          eprintln!(
            "Execution of command '{}' failed, error: {}",
            it.name(),
            error
          );
          process::exit(1);
        } else {
          result
        }
      })
      .expect("Valid subcommand")?;
  } else {
    panic!("Unexpected cli command provided, check --help for details")
  }

  Ok(())
}

pub fn setup_subcommands() -> Vec<Box<dyn GenericCommand>> {
  vec![
    // Archive:
    UnpackArchiveCommand::new_box(),
    // Gamedata:
    VerifyGamedataCommand::new_box(),
    // LTX:
    FormatLtxCommand::new_box(),
    VerifyLtxCommand::new_box(),
    // OGF:
    InfoOgfCommand::new_box(),
    // OMF:
    InfoOmfCommand::new_box(),
    // Particles:
    InfoParticlesCommand::new_box(),
    PackParticlesFileCommand::new_box(),
    RepackParticlesCommand::new_box(),
    ReUnpackParticlesCommand::new_box(),
    UnpackParticlesCommand::new_box(),
    VerifyParticlesFileCommand::new_box(),
    // Spawn:
    InfoSpawnCommand::new_box(),
    PackSpawnFileCommand::new_box(),
    RepackSpawnCommand::new_box(),
    UnpackSpawnFileCommand::new_box(),
    VerifySpawnFileCommand::new_box(),
    // Textures:
    InfoDdsCommand::new_box(),
    PackEquipmentIconsCommand::new_box(),
    PackTextureDescriptionCommand::new_box(),
    UnpackEquipmentIconsCommand::new_box(),
    UnpackTextureDescriptionCommand::new_box(),
    // Translations:
    BuildTranslationsCommand::new_box(),
    InitializeTranslationsCommand::new_box(),
    ParseTranslationsCommand::new_box(),
    VerifyTranslationsCommand::new_box(),
  ]
}

/// Configure environment logger, fallback to info level.
pub fn setup_logger() {
  let mut logger: Builder = env_logger::builder();

  if let Ok(rust_log) = env::var("RUST_LOG") {
    logger.parse_filters(&rust_log);
  } else {
    match cfg!(debug_assertions) {
      true => logger.filter_level(LevelFilter::Warn),
      false => logger.filter_level(LevelFilter::Error),
    };
  }

  logger.default_format();
  logger.init();
}

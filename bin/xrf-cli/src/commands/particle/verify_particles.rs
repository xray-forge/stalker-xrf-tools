use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{ParticlesFile, XRayByteOrder};
use xray_error::XRayError;

#[derive(Default)]
pub struct VerifyParticlesFileCommand;

impl GenericCommand for VerifyParticlesFileCommand {
  fn name(&self) -> &'static str {
    "verify-particles"
  }

  /// Create command for verifying of particle file.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to verify provided particle file")
      .arg(
        Arg::new("path")
          .help("Path to particle.xr file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("unpacked")
          .help("Whether should verify unpacked particle")
          .short('u')
          .long("unpacked")
          .required(false)
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Verify particle file based on provided arguments.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<_>("path")
      .expect("Expected valid path to be provided");

    let unpacked: bool = matches.get_flag("unpacked");

    log::info!(
      "Verify particle file {}, unpacked: {}",
      path.display(),
      unpacked
    );

    let particles_file_result: CommandResult<ParticlesFile> = if unpacked {
      ParticlesFile::import_from_path(path).map_err(Into::into)
    } else {
      ParticlesFile::read_from_path::<XRayByteOrder, _>(path).map_err(Into::into)
    };

    match particles_file_result {
      Ok(_) => {
        log::info!("Provided particle file is valid");

        // todo: Check nested textures.
        // todo: Check nested textures.
        // todo: Check nested textures.

        Ok(())
      }
      Err(error) => {
        log::error!("Provided particle file is invalid: {}", error);

        Err(
          XRayError::new_parsing_error(format!("Verification of particle file failed: {}", error))
            .into(),
        )
      }
    }
  }
}

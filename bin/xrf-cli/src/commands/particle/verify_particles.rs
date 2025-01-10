use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{DatabaseParseError, ParticlesFile, XRayByteOrder};

#[derive(Default)]
pub struct VerifyParticlesFileCommand;

impl GenericCommand for VerifyParticlesFileCommand {
  fn name(&self) -> &'static str {
    "verify-particle"
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
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let unpacked: bool = matches.get_flag("unpacked");

    log::info!("Verify particle file {:?}, unpacked: {unpacked}", path);

    let particles_file_result: CommandResult<ParticlesFile> = if unpacked {
      ParticlesFile::import_from_path(path).map_err(Into::into)
    } else {
      ParticlesFile::read_from_path::<XRayByteOrder>(path).map_err(Into::into)
    };

    match particles_file_result {
      Ok(_) => {
        log::info!("Provided particle file is valid");

        Ok(())
      }
      Err(error) => {
        log::error!("Provided particle file is invalid: {}", error);

        Err(
          DatabaseParseError::new_database_error(format!(
            "Verification of particle file failed: {}",
            error
          ))
          .into(),
        )
      }
    }
  }
}

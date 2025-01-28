use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{DatabaseError, SpawnFile, XRayByteOrder};

#[derive(Default)]
pub struct VerifySpawnFileCommand;

impl GenericCommand for VerifySpawnFileCommand {
  fn name(&self) -> &'static str {
    "verify-spawn"
  }

  /// Create command for verifying of spawn file.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to verify provided spawn file")
      .arg(
        Arg::new("path")
          .help("Path to spawn file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Verify *.spawn file based on provided arguments.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    log::info!("Verify spawn file {}", path.display());

    match SpawnFile::read_from_path::<XRayByteOrder>(path) {
      Ok(_) => {
        log::info!("Provided spawn file is valid");

        Ok(())
      }
      Err(error) => {
        log::error!("Provided spawn file is invalid: {}", error);

        Err(
          DatabaseError::new_parse_error(format!("Verification of spawn file failed: {}", error))
            .into(),
        )
      }
    }
  }
}

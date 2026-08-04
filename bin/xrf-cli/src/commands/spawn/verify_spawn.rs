use crate::generic_command::{CommandResult, GenericCommand};
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::path::PathBuf;
use xray_db::{SpawnFile, XRayByteOrder};
use xray_error::XRayError;

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
      .arg(
        Arg::new("silent")
          .help("Turn off logging")
          .long("silent")
          .required(false)
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("verbose")
          .help("Turn on verbose logging")
          .short('v')
          .long("verbose")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Verify *.spawn file based on provided arguments.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<_>("path")
      .expect("Expected valid path to be provided");

    log::info!("Verify spawn file {}", path.display());

    match SpawnFile::read_from_path::<XRayByteOrder, _>(path) {
      Ok(_) => {
        log::info!("Provided spawn file is valid");

        Ok(())
      }
      Err(error) => {
        log::error!("Provided spawn file is invalid: {}", error);

        Err(
          XRayError::new_parsing_error(format!("Verification of spawn file failed: {}", error))
            .into(),
        )
      }
    }
  }
}

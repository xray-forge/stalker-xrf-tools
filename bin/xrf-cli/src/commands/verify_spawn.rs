use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{DatabaseParseError, DatabaseResult, SpawnFile, XRayByteOrder};

pub struct VerifySpawnFileCommand {}

impl VerifySpawnFileCommand {
  pub const NAME: &'static str = "verify-spawn";

  /// Create command for verifying of spawn file.
  pub fn init() -> Command {
    Command::new(Self::NAME)
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
  pub fn execute(matches: &ArgMatches) -> DatabaseResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    log::info!("Verify spawn file {:?}", path);

    match SpawnFile::read_from_path::<XRayByteOrder>(path) {
      Ok(_) => {
        log::info!("Provided spawn file is valid");

        Ok(())
      }
      Err(error) => {
        log::error!("Provided spawn file is invalid: {}", error);

        Err(DatabaseParseError::new_database_error(format!(
          "Verification of spawn file failed: {}",
          error
        )))
      }
    }
  }
}

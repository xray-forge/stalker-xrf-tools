use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

/// Create command for verifying of spawn file.
pub fn create_verify_spawn_file_command() -> Command {
  Command::new("verify-spawn")
    .about("Command to verify provided *.spawn file")
    .arg(
      Arg::new("path")
        .help("Path to *.spawn file")
        .short('p')
        .long("path")
        .required(true)
        .value_parser(value_parser!(PathBuf)),
    )
}

/// Verify *.spawn file based on provided arguments.
pub fn verify_spawn_file(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid path to be provided");

  log::info!("Verify spawn file {:?}", path);

  match SpawnFile::read_from_path::<SpawnByteOrder>(path) {
    Ok(_) => log::info!("Provided spawn file is valid"),
    Err(error) => {
      log::error!("Provided spawn file is invalid: {:?}", error);
      panic!("{:?}", error);
    }
  }
}

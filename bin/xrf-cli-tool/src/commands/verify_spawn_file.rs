use clap::ArgMatches;
use std::path::PathBuf;
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

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

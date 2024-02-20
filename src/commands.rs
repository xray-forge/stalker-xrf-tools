use clap::ArgMatches;
use std::path::PathBuf;
use xray_spawn::spawn_file::SpawnFile;
use xray_spawn::types::SpawnByteOrder;

/// Unpack provided *.spawn file and validate it.
pub fn unpack_spawn_file(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid path to be provided");

  log::info!("Starting parsing spawn file in, target path {:?}", path);

  let spawn_file: Box<SpawnFile> =
    Box::new(SpawnFile::read_from_path::<SpawnByteOrder>(path).unwrap());

  log::info!("Spawn file: {:?}", spawn_file);
}

/// Pack *.spawn file based on provided arguments.
pub fn pack_spawn_file(_: &ArgMatches) {
  todo!("Implement pack command");
}

/// Verify *.spawn file based on provided arguments.
pub fn verify_spawn_file(_: &ArgMatches) {
  todo!("Implement verify command");
}

/// Lint ltx file or folder based on provided arguments.
pub fn format_ltx(_: &ArgMatches) {
  todo!("Implement ltx-format command");
}

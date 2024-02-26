use clap::ArgMatches;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

/// Unpack provided *.spawn file and validate it.
pub fn unpack_spawn_file(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid path to be provided");

  log::info!("Starting parsing spawn file in, target path {:?}", path);

  let started_at: Instant = Instant::now();

  let spawn_file: Box<SpawnFile> =
    Box::new(SpawnFile::read_from_path::<SpawnByteOrder>(path).unwrap());

  let read_duration: Duration = started_at.elapsed();

  spawn_file
    .write_to_path::<SpawnByteOrder>(&PathBuf::from("test.spawn"))
    .expect("Correctly written spawn file");

  let write_duration: Duration = started_at.elapsed() - read_duration;

  log::info!("Read spawn file took: {:?}ms", read_duration.as_millis());
  log::info!("Write spawn file took: {:?}ms", write_duration.as_millis());
}

/// Pack *.spawn file based on provided arguments.
pub fn pack_spawn_file(_: &ArgMatches) {
  todo!("Implement pack command");
}

/// Verify *.spawn file based on provided arguments.
pub fn verify_spawn_file(_: &ArgMatches) {
  todo!("Implement verify command");
}

/// Unpack xray engine database archive.
pub fn unpack_archive(_: &ArgMatches) {
  todo!("Implement unpack archive command");
}

/// Lint ltx file or folder based on provided arguments.
pub fn format_ltx(_: &ArgMatches) {
  todo!("Implement ltx-format command");
}

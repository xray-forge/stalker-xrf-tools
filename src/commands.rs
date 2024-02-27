use clap::ArgMatches;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, io};
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

/// Repack provided *.spawn file and validate it.
pub fn repack_spawn_file(matches: &ArgMatches) -> io::Result<()> {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  let destination: &PathBuf = matches
    .get_one::<PathBuf>("dest")
    .expect("Expected valid output path to be provided");

  log::info!("Starting parsing spawn file {:?}", path);
  log::info!("Repack into {:?}", destination);

  let started_at: Instant = Instant::now();

  let spawn_file: Box<SpawnFile> =
    Box::new(SpawnFile::read_from_path::<SpawnByteOrder>(path).unwrap());

  let read_duration: Duration = started_at.elapsed();

  spawn_file
    .write_to_path::<SpawnByteOrder>(destination)
    .expect("Correctly written spawn file");

  let write_duration: Duration = started_at.elapsed() - read_duration;

  log::info!("Read spawn file took: {:?}ms", read_duration.as_millis());
  log::info!("Write spawn file took: {:?}ms", write_duration.as_millis());

  log::info!("Spawn file was unpacked into {:?}", destination);

  Ok(())
}

/// Unpack provided *.spawn file.
pub fn unpack_spawn_file(matches: &ArgMatches) -> io::Result<()> {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid path to be provided");

  let destination: &PathBuf = matches
    .get_one::<PathBuf>("dest")
    .expect("Expected valid output path to be provided");

  let force: bool = matches.get_flag("force");

  log::info!("Starting parsing spawn file {:?}", path);
  log::info!("Unpack destination {:?}", destination);

  // Apply force flag and delete existing directories.
  if force && destination.exists() && destination.is_dir() {
    fs::remove_dir_all(destination)?;
  }

  // Re-validate that provided output can be used.
  if destination.exists() && destination.is_dir() {
    return Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      "Unpack output directory already exists, use --force to prune destination folder.",
    ));
  }

  let started_at: Instant = Instant::now();

  let spawn_file: Box<SpawnFile> =
    Box::new(SpawnFile::read_from_path::<SpawnByteOrder>(path).unwrap());

  let read_duration: Duration = started_at.elapsed();

  spawn_file.export_to_path::<SpawnByteOrder>(destination)?;

  let unpack_duration: Duration = started_at.elapsed() - read_duration;

  log::info!("Read spawn file took: {:?}ms", read_duration.as_millis());
  log::info!(
    "Export spawn file took: {:?}ms",
    unpack_duration.as_millis()
  );

  Ok(())
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

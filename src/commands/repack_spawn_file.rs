use clap::ArgMatches;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};
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

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, io};
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

/// Create command packing of spawn file.
pub fn create_pack_spawn_file_command() -> Command {
  Command::new("pack-spawn")
    .about("Command to pack unpacked spawn files into single *.spawn")
    .arg(
      Arg::new("path")
        .help("Path to unpacked spawn file folder")
        .short('p')
        .long("path")
        .required(true)
        .value_parser(value_parser!(PathBuf)),
    )
    .arg(
      Arg::new("dest")
        .help("Path to resulting packed *.spawn file")
        .short('d')
        .long("dest")
        .default_value("unpacked")
        .value_parser(value_parser!(PathBuf)),
    )
    .arg(
      Arg::new("force")
        .help("Whether existing packed spawwn should be pruned if destination folder exists")
        .short('f')
        .long("force")
        .required(false)
        .action(ArgAction::SetTrue),
    )
}

/// Pack *.spawn file based on provided arguments.
pub fn pack_spawn_file(matches: &ArgMatches) -> io::Result<()> {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid path to be provided");

  let destination: &PathBuf = matches
    .get_one::<PathBuf>("dest")
    .expect("Expected valid output path to be provided");

  let force: bool = matches.get_flag("force");

  log::info!("Starting packing spawn file {:?}", path);
  log::info!("Pack destination {:?}", destination);

  // Apply force flag and delete existing spawn output.
  if force && destination.exists() && destination.is_file() {
    fs::remove_file(destination)?;
  }

  // Re-validate that provided output can be used.
  if destination.exists() && destination.is_file() {
    return Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      "Pack output file already exists, use --force to prune destination",
    ));
  }

  let started_at: Instant = Instant::now();
  let spawn_file: SpawnFile = SpawnFile::import_from_path::<SpawnByteOrder>(path).unwrap();
  let read_duration: Duration = started_at.elapsed();

  spawn_file.write_to_path::<SpawnByteOrder>(destination)?;

  let write_duration: Duration = started_at.elapsed() - read_duration;

  log::info!("Read spawn file took: {:?}ms", read_duration.as_millis());
  log::info!(
    "Writing packed spawn file took: {:?}ms",
    write_duration.as_millis()
  );

  Ok(())
}

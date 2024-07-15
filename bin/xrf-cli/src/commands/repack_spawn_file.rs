use clap::{value_parser, Arg, ArgMatches, Command};
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

/// Add command for repack of spawn file.
pub fn add_repack_spawn_file_command(command: Command) -> Command {
  command.subcommand(
    Command::new("repack-spawn")
      .about("Command to repack provided *.spawn into another file")
      .arg(
        Arg::new("path")
          .help("Path to *.spawn file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("dest")
          .help("Path to resulting *.spawn file")
          .short('d')
          .long("dest")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      ),
  )
}

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
  let spawn_file: SpawnFile = SpawnFile::read_from_path::<SpawnByteOrder>(path).unwrap();
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

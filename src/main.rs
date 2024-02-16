mod setup;
mod spawn;

use crate::setup::setup_logger;
use crate::spawn::spawn_file::SpawnFile;
use clap::Parser;

#[derive(Parser)]
struct Arguments {
  /// Command to execute - pack/unpack.
  command: String,
  /// Path to spawn file for processing.
  path: std::path::PathBuf,
}

fn main() -> () {
  setup_logger();

  let arguments: Arguments = Arguments::parse();

  log::info!(
    "Starting spawn cli in {:?} mode, target path {:?}",
    arguments.command,
    arguments.path
  );

  let spawn_file: SpawnFile = SpawnFile::from_path(&arguments.path).unwrap();

  log::info!("Spawn file: {:?}", spawn_file);
}

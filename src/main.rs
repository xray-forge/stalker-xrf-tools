mod setup;

use crate::setup::setup_logger;
use clap::Parser;
use xray_spawn::spawn_file::SpawnFile;

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

  log::info!("Spawn file: {:?}", spawn_file,);
}

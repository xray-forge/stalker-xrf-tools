use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

/// Create command for printing spawn file info.
pub fn create_info_spawn_file_command() -> Command {
  Command::new("info-spawn")
    .about("Command to print information about provided *.spawn file")
    .arg(
      Arg::new("path")
        .help("Path to *.spawn file")
        .short('p')
        .long("path")
        .required(true)
        .value_parser(value_parser!(PathBuf)),
    )
}

/// Print information about spawn file.
pub fn info_spawn_file(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid path to be provided");

  log::info!("Verify spawn file {:?}", path);

  let spawn_file: SpawnFile = match SpawnFile::read_from_path::<SpawnByteOrder>(path) {
    Ok(file) => file,
    Err(error) => {
      log::error!("Provided spawn file is invalid: {:?}", error);
      panic!("{:?}", error);
    }
  };

  log::info!("Spawn file information:");

  log::info!("Version: {}", spawn_file.header.version);
  log::info!("GUID: {}", spawn_file.header.guid);
  log::info!("Levels count: {}", spawn_file.header.levels_count);
  log::info!("Objects count: {}", spawn_file.header.objects_count);

  log::info!(
    "Artefact spawn points: {}",
    spawn_file.artefact_spawn.nodes.len()
  );

  log::info!("Patrols: {}", spawn_file.patrols.patrols.len());

  log::info!("Level version: {}", spawn_file.graphs.header.version);
  log::info!(
    "Level graph vertices: {}",
    spawn_file.graphs.header.vertices_count
  );
  log::info!(
    "Level graph points: {}",
    spawn_file.graphs.header.points_count
  );
  log::info!(
    "Level graph edges: {}",
    spawn_file.graphs.header.edges_count
  );
}

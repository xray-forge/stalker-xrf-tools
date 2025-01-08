use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{DatabaseResult, SpawnByteOrder, SpawnFile};

pub struct InfoSpawnCommand {}

impl InfoSpawnCommand {
  pub const NAME: &'static str = "info-spawn";

  /// Create command for printing spawn file info.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to print information about provided spawn file")
      .arg(
        Arg::new("path")
          .help("Path to spawn file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Print information about spawn file.
  pub fn execute(matches: &ArgMatches) -> DatabaseResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    println!("Read spawn file {:?}", path);

    let spawn_file: SpawnFile = SpawnFile::read_from_path::<SpawnByteOrder>(path)?;

    println!("Spawn file information:");

    println!("Version: {}", spawn_file.header.version);
    println!("GUID: {}", spawn_file.header.guid);
    println!("Levels count: {}", spawn_file.header.levels_count);
    println!("Objects count: {}", spawn_file.header.objects_count);

    println!(
      "Artefact spawn points: {}",
      spawn_file.artefact_spawn.nodes.len()
    );

    println!("Patrols: {}", spawn_file.patrols.patrols.len());

    println!("Level version: {}", spawn_file.graphs.header.version);
    println!(
      "Level graph vertices: {}",
      spawn_file.graphs.header.vertices_count
    );
    println!(
      "Level graph points: {}",
      spawn_file.graphs.header.points_count
    );
    println!(
      "Level graph edges: {}",
      spawn_file.graphs.header.edges_count
    );

    Ok(())
  }
}

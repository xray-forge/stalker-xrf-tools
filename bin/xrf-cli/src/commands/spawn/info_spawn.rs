use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{SpawnFile, XRayByteOrder};

#[derive(Default)]
pub struct InfoSpawnCommand;

impl GenericCommand for InfoSpawnCommand {
  fn name(&self) -> &'static str {
    "info-spawn"
  }

  /// Create command for printing spawn file info.
  fn init(&self) -> Command {
    Command::new(self.name())
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
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<_>("path")
      .expect("Expected valid path to be provided");

    println!("Read spawn file {}", path.display());

    let spawn_file: Box<SpawnFile> = Box::new(SpawnFile::read_from_path::<XRayByteOrder, _>(path)?);

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

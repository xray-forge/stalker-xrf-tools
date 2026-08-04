use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::path::PathBuf;
use xray_db::{SpawnFile, XRayByteOrder};
use xray_output::OutputOptions;

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
      .arg(
        Arg::new("silent")
          .help("Disable any logging")
          .short('s')
          .long("silent")
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("verbose")
          .help("Turn on verbose logging")
          .short('v')
          .long("verbose")
          .action(ArgAction::SetTrue),
      )
  }

  /// Print information about spawn file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<_>("path")
      .expect("Expected valid path to be provided");

    let output: OutputOptions =
      TerminalOutput::from_options(matches.get_flag("silent"), matches.get_flag("verbose"));

    xray_output::info!(output, "Read spawn file {}", path.display());

    let spawn_file: Box<SpawnFile> = Box::new(SpawnFile::read_from_path::<XRayByteOrder, _>(path)?);

    xray_output::info!(output, "Spawn file information:");

    xray_output::info!(output, "Version: {}", spawn_file.header.version);
    xray_output::info!(output, "GUID: {}", spawn_file.header.guid);
    xray_output::info!(output, "Levels count: {}", spawn_file.header.levels_count);
    xray_output::info!(output, "Objects count: {}", spawn_file.header.objects_count);

    xray_output::info!(
      output,
      "Artefact spawn points: {}",
      spawn_file.artefact_spawn.nodes.len()
    );

    xray_output::info!(output, "Patrols: {}", spawn_file.patrols.patrols.len());

    xray_output::info!(
      output,
      "Level version: {}",
      spawn_file.graphs.header.version
    );
    xray_output::info!(
      output,
      "Level graph vertices: {}",
      spawn_file.graphs.header.vertices_count
    );
    xray_output::info!(
      output,
      "Level graph points: {}",
      spawn_file.graphs.header.points_count
    );
    xray_output::info!(
      output,
      "Level graph edges: {}",
      spawn_file.graphs.header.edges_count
    );

    Ok(())
  }
}

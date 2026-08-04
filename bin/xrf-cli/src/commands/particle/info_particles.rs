use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::path::PathBuf;
use xray_db::{ParticlesFile, XRayByteOrder};
use xray_output::OutputOptions;

#[derive(Default)]
pub struct InfoParticlesCommand;

impl GenericCommand for InfoParticlesCommand {
  fn name(&self) -> &'static str {
    "info-particles"
  }

  /// Create command for printing particle file info.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to print information about provided particle file")
      .arg(
        Arg::new("path")
          .help("Path to particle file")
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

  /// Print information about particle file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<_>("path")
      .expect("Expected valid path to be provided");

    let output: OutputOptions =
      TerminalOutput::from_options(matches.get_flag("silent"), matches.get_flag("verbose"));

    xray_output::info!(output, "Read particle file {}", path.display());

    let particles_file: Box<ParticlesFile> =
      Box::new(ParticlesFile::read_from_path::<XRayByteOrder, _>(path)?);

    xray_output::info!(output, "Particles file information:");

    xray_output::info!(output, "Version: {}", particles_file.header.version);
    xray_output::info!(
      output,
      "Effects count: {}",
      particles_file.effects.effects.len()
    );
    xray_output::info!(
      output,
      "Groups count: {}",
      particles_file.groups.groups.len()
    );

    Ok(())
  }
}

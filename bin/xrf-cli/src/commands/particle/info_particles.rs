use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{ParticlesFile, XRayByteOrder};

#[derive(Default)]
pub struct InfoParticlesCommand;

impl GenericCommand for InfoParticlesCommand {
  fn name(&self) -> &'static str {
    "info-particle"
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
  }

  /// Print information about particle file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    println!("Read particle file {:?}", path);

    let particles_file: ParticlesFile = ParticlesFile::read_from_path::<XRayByteOrder>(path)?;

    println!("Particles file information:");

    println!("Version: {}", particles_file.header.version);
    println!("Effects count: {}", particles_file.effects.effects.len());
    println!("Groups count: {}", particles_file.groups.groups.len());

    Ok(())
  }
}

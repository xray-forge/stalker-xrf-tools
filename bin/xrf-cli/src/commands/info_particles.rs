use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{DatabaseResult, ParticlesFile, XRayByteOrder};

pub struct InfoParticlesCommand {}

impl InfoParticlesCommand {
  pub const NAME: &'static str = "info-particles";

  /// Create command for printing particles file info.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to print information about provided particles file")
      .arg(
        Arg::new("path")
          .help("Path to particles file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Print information about particles file.
  pub fn execute(matches: &ArgMatches) -> DatabaseResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    println!("Read particles file {:?}", path);

    let particles_file: ParticlesFile = ParticlesFile::read_from_path::<XRayByteOrder>(path)?;

    println!("Particles file information:");

    println!("Version: {}", particles_file.header.version);
    println!("Effects count: {}", particles_file.effects.effects.len());
    println!("Groups count: {}", particles_file.groups.groups.len());

    Ok(())
  }
}

use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use xray_db::ParticlesFile;

#[derive(Default)]
pub struct ReUnpackParticlesCommand;

impl GenericCommand for ReUnpackParticlesCommand {
  fn name(&self) -> &'static str {
    "re-unpack-particle"
  }

  /// Create command for re-unpack of particle file.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to re-unpack provided particle directory into another directory")
      .arg(
        Arg::new("path")
          .help("Path to unpacked particle directory")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("dest")
          .help("Path to resulting unpacked particle")
          .short('d')
          .long("dest")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Re-unpack provided particle dir and validate it.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid input path to be provided");

    let destination: &PathBuf = matches
      .get_one::<PathBuf>("dest")
      .expect("Expected valid output path to be provided");

    log::info!("Starting importing particle file {:?}", path);
    log::info!("Re-unpack into {:?}", destination);

    let started_at: Instant = Instant::now();
    let particles_file: ParticlesFile = ParticlesFile::import_from_path(path)?;
    let import_duration: Duration = started_at.elapsed();

    particles_file.export_to_path(destination)?;

    let export_duration: Duration = started_at.elapsed() - import_duration;

    log::info!(
      "Import particle file took: {:?}ms",
      import_duration.as_millis()
    );
    log::info!(
      "Export particle file took: {:?}ms",
      export_duration.as_millis()
    );

    log::info!("Particles file was re-unpacked into {:?}", destination);

    Ok(())
  }
}

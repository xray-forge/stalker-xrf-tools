use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use xray_db::{ParticlesFile, XRayByteOrder};

#[derive(Default)]
pub struct RepackParticlesCommand;

impl GenericCommand for RepackParticlesCommand {
  fn name(&self) -> &'static str {
    "repack-particles"
  }

  /// Create command for repack of particle file.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to repack provided particle.xr into another file")
      .arg(
        Arg::new("path")
          .help("Path to particle file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("dest")
          .help("Path to resulting particle file")
          .short('d')
          .long("dest")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Repack provided particle file and validate it.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<_>("path")
      .expect("Expected valid input path to be provided");

    let destination: &PathBuf = matches
      .get_one::<_>("dest")
      .expect("Expected valid output path to be provided");

    log::info!("Starting parsing particle file {}", path.display());
    log::info!("Repack into {}", destination.display());

    let started_at: Instant = Instant::now();
    let particles_file: Box<ParticlesFile> =
      Box::new(ParticlesFile::read_from_path::<XRayByteOrder, _>(path)?);
    let read_duration: Duration = started_at.elapsed();

    particles_file.write_to_path::<XRayByteOrder, _>(destination)?;

    let write_duration: Duration = started_at.elapsed() - read_duration;

    log::info!("Read particle file took: {}ms", read_duration.as_millis());
    log::info!("Write particle file took: {}ms", write_duration.as_millis());

    log::info!("Particles file was repacked into {}", destination.display());

    Ok(())
  }
}

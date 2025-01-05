use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use xray_db::{DatabaseResult, ParticlesFile};

pub struct ReUnpackParticlesCommand {}

impl ReUnpackParticlesCommand {
  pub const NAME: &'static str = "re-unpack-particles";

  /// Create command for re-unpack of particles file.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to re-unpack provided particles directory into another directory")
      .arg(
        Arg::new("path")
          .help("Path to unpacked particles directory")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("dest")
          .help("Path to resulting unpacked particles")
          .short('d')
          .long("dest")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Re-unpack provided particles dir and validate it.
  pub fn execute(matches: &ArgMatches) -> DatabaseResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid input path to be provided");

    let destination: &PathBuf = matches
      .get_one::<PathBuf>("dest")
      .expect("Expected valid output path to be provided");

    log::info!("Starting importing particles file {:?}", path);
    log::info!("Re-unpack into {:?}", destination);

    let started_at: Instant = Instant::now();
    let particles_file: ParticlesFile = ParticlesFile::import_from_path(path)?;
    let import_duration: Duration = started_at.elapsed();

    particles_file.export_to_path(destination)?;

    let export_duration: Duration = started_at.elapsed() - import_duration;

    log::info!(
      "Import particles file took: {:?}ms",
      import_duration.as_millis()
    );
    log::info!(
      "Export particles file took: {:?}ms",
      export_duration.as_millis()
    );

    log::info!("Particles file was re-unpacked into {:?}", destination);

    Ok(())
  }
}

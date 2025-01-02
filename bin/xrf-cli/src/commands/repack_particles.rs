use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use xray_db::particles_file::particles_file::ParticlesFile;
use xray_db::types::{DatabaseResult, ParticlesByteOrder};

pub struct RepackParticlesCommand {}

impl RepackParticlesCommand {
  pub const NAME: &'static str = "repack-particles";

  /// Create command for repack of particles file.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to repack provided particles.xr into another file")
      .arg(
        Arg::new("path")
          .help("Path to particles.xr file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("dest")
          .help("Path to resulting *.xr file")
          .short('d')
          .long("dest")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Repack provided particles file and validate it.
  pub fn execute(matches: &ArgMatches) -> DatabaseResult<()> {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid input path to be provided");

    let destination: &PathBuf = matches
      .get_one::<PathBuf>("dest")
      .expect("Expected valid output path to be provided");

    log::info!("Starting parsing particles file {:?}", path);
    log::info!("Repack into {:?}", destination);

    let started_at: Instant = Instant::now();
    let particles_file: ParticlesFile = ParticlesFile::read_from_path::<ParticlesByteOrder>(path)?;
    let read_duration: Duration = started_at.elapsed();

    particles_file.write_to_path::<ParticlesByteOrder>(destination)?;

    let write_duration: Duration = started_at.elapsed() - read_duration;

    log::info!(
      "Read particles file took: {:?}ms",
      read_duration.as_millis()
    );
    log::info!(
      "Write particles file took: {:?}ms",
      write_duration.as_millis()
    );

    log::info!("Particles file was repacked into {:?}", destination);

    Ok(())
  }
}

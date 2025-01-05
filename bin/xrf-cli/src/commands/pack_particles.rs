use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, io};
use xray_db::{DatabaseResult, ParticlesByteOrder, ParticlesFile};

pub struct PackParticlesFileCommand {}

impl PackParticlesFileCommand {
  pub const NAME: &'static str = "pack-particles";

  /// Create command packing of particles file.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to pack unpacked particles files into single particles.xr")
      .arg(
        Arg::new("path")
          .help("Path to unpacked particles file folder")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("dest")
          .help("Path to resulting packed *.xr file")
          .short('d')
          .long("dest")
          .default_value("unpacked")
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("force")
          .help("Whether existing packed particles should be pruned if destination folder exists")
          .short('f')
          .long("force")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Pack particles file based on provided arguments.
  pub fn execute(matches: &ArgMatches) -> DatabaseResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let destination: &PathBuf = matches
      .get_one::<PathBuf>("dest")
      .expect("Expected valid output path to be provided");

    let force: bool = matches.get_flag("force");

    log::info!("Starting packing particles file {:?}", path);
    log::info!("Pack destination {:?}", destination);

    // Apply force flag and delete existing particles output.
    if force && destination.exists() && destination.is_file() {
      fs::remove_file(destination)?;
    }

    // Re-validate that provided output can be used.
    if destination.exists() && destination.is_file() {
      return Err(
        io::Error::new(
          io::ErrorKind::AlreadyExists,
          "Pack output file already exists, use --force to prune destination",
        )
        .into(),
      );
    }

    let started_at: Instant = Instant::now();
    let particles_file: ParticlesFile = ParticlesFile::import_from_path(path)?;
    let read_duration: Duration = started_at.elapsed();

    particles_file.write_to_path::<ParticlesByteOrder>(destination)?;

    let write_duration: Duration = started_at.elapsed() - read_duration;

    log::info!(
      "Read particles file took: {:?}ms",
      read_duration.as_millis()
    );
    log::info!(
      "Writing packed particles file took: {:?}ms",
      write_duration.as_millis()
    );

    Ok(())
  }
}

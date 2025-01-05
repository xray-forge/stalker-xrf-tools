use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, io};
use xray_db::{DatabaseResult, ParticlesByteOrder, ParticlesFile};

pub struct UnpackParticlesCommand {}

impl UnpackParticlesCommand {
  pub const NAME: &'static str = "unpack-particles";

  /// Create command to unpack particles xr file.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to unpack provided particles.xr into separate files")
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
          .help("Path to folder for exporting")
          .short('d')
          .long("dest")
          .default_value("unpacked")
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("force")
          .help("Whether existing unpacked data should be pruned if destination folder exists")
          .short('f')
          .long("force")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Unpack provided particles file.
  pub fn execute(matches: &ArgMatches) -> DatabaseResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let destination: &PathBuf = matches
      .get_one::<PathBuf>("dest")
      .expect("Expected valid output path to be provided");

    let force: bool = matches.get_flag("force");

    log::info!("Starting particles spawn file {:?}", path);
    log::info!("Unpack destination {:?}", destination);

    // Apply force flag and delete existing directories.
    if force && destination.exists() && destination.is_dir() {
      fs::remove_dir_all(destination)?;
    }

    // Re-validate that provided output can be used.
    if destination.exists() && destination.is_dir() {
      return Err(
        io::Error::new(
          io::ErrorKind::AlreadyExists,
          "Unpack output directory already exists, use --force to prune destination folder",
        )
        .into(),
      );
    }

    let started_at: Instant = Instant::now();
    let particles_file: ParticlesFile = ParticlesFile::read_from_path::<ParticlesByteOrder>(path)?;
    let read_duration: Duration = started_at.elapsed();

    particles_file.export_to_path(destination)?;

    let unpack_duration: Duration = started_at.elapsed() - read_duration;

    log::info!(
      "Read particles file took: {:?}ms",
      read_duration.as_millis()
    );
    log::info!(
      "Export particles file took: {:?}ms",
      unpack_duration.as_millis()
    );

    Ok(())
  }
}

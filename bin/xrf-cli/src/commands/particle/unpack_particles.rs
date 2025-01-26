use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, io};
use xray_db::{ParticlesFile, XRayByteOrder};

#[derive(Default)]
pub struct UnpackParticlesCommand;

impl GenericCommand for UnpackParticlesCommand {
  fn name(&self) -> &'static str {
    "unpack-particles"
  }

  /// Create command to unpack particle xr file.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to unpack provided particle.xr into separate files")
      .arg(
        Arg::new("path")
          .help("Path to particle.xr file")
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

  /// Unpack provided particle file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let destination: &PathBuf = matches
      .get_one::<PathBuf>("dest")
      .expect("Expected valid output path to be provided");

    let force: bool = matches.get_flag("force");

    log::info!("Starting particle spawn file {:?}", path);
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
    let particles_file: ParticlesFile =
      ParticlesFile::read_from_path::<XRayByteOrder, &PathBuf>(path)?;
    let read_duration: Duration = started_at.elapsed();

    particles_file.export_to_path(destination)?;

    let unpack_duration: Duration = started_at.elapsed() - read_duration;

    log::info!("Read particle file took: {:?}ms", read_duration.as_millis());
    log::info!(
      "Export particle file took: {:?}ms",
      unpack_duration.as_millis()
    );

    Ok(())
  }
}

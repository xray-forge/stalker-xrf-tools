use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{fs, io};
use xray_db::{ParticlesFile, XRayByteOrder};

#[derive(Default)]
pub struct PackParticlesFileCommand;

impl GenericCommand for PackParticlesFileCommand {
  fn name(&self) -> &'static str {
    "pack-particles"
  }

  /// Create command packing of particle file.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to pack unpacked particle files into single particle.xr")
      .arg(
        Arg::new("path")
          .help("Path to unpacked particle file folder")
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
          .help("Whether existing packed particle should be pruned if destination folder exists")
          .short('f')
          .long("force")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Pack particle file based on provided arguments.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let destination: &PathBuf = matches
      .get_one::<PathBuf>("dest")
      .expect("Expected valid output path to be provided");

    let force: bool = matches.get_flag("force");

    log::info!("Starting packing particle file {}", path.display());
    log::info!("Pack destination {}", destination.display());

    // Apply force flag and delete existing particle output.
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
    let particles_file: Box<ParticlesFile> = Box::new(ParticlesFile::import_from_path(path)?);
    let read_duration: Duration = started_at.elapsed();

    particles_file.write_to_path::<XRayByteOrder, &PathBuf>(destination)?;

    let write_duration: Duration = started_at.elapsed() - read_duration;

    log::info!("Read particle file took: {}ms", read_duration.as_millis());
    log::info!(
      "Writing packed particle file took: {}ms",
      write_duration.as_millis()
    );

    Ok(())
  }
}

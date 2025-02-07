use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::env;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use xray_archive::{ArchiveProject, ArchiveUnpackResult};

#[derive(Default)]
pub struct UnpackArchiveCommand;

impl GenericCommand for UnpackArchiveCommand {
  fn name(&self) -> &'static str {
    "unpack-archive"
  }

  /// Create command to unpack archive.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to unpack provided *.db into separate files")
      .arg(
        Arg::new("path")
          .help("Path to *.db file")
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
        Arg::new("parallel")
          .help("Count of parallel threads for unpack")
          .long("parallel")
          .default_value("32")
          .value_parser(value_parser!(usize)),
      )
  }

  /// Unpack xray engine database archive.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<_>("path")
      .expect("Expected valid path to be provided");

    let destination: &PathBuf = matches
      .get_one::<_>("dest")
      .expect("Expected valid output path to be provided");

    let destination: PathBuf = if destination.is_relative() {
      env::current_dir()?.join(destination)
    } else {
      destination.clone()
    };

    let parallel: usize = *matches
      .get_one::<usize>("parallel")
      .expect("Expected valid parallel threads count to be provided");

    log::info!("Unpack source: {}", path.display());
    log::info!("Unpack destination: {}", destination.display());

    let archive_project: Box<ArchiveProject> = Box::new(ArchiveProject::new(path)?);

    log::info!(
      "Summary: {} archive(s), {} file(s), {:.3} MB compressed, {:.3} MB real",
      archive_project.archives.len(),
      archive_project.files.len(),
      (archive_project.get_compressed_size() as f64) / 1024.0 / 1024.0,
      (archive_project.get_real_size() as f64) / 1024.0 / 1024.0,
    );

    log::info!("Unpacking files, parallel {parallel}");

    let result: ArchiveUnpackResult =
      Runtime::new()?.block_on(archive_project.unpack_parallel(&destination, parallel))?;

    log::info!(
      "Unpacked archive, took {} sec (preparation {} sec, unpack {} sec)",
      result.duration as f64 / 1000.0,
      result.prepare_duration as f64 / 1000.0,
      result.unpack_duration as f64 / 1000.0,
    );

    Ok(())
  }
}

use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::{Path, PathBuf};
use xray_db::{OmfFile, XRayByteOrder};

#[derive(Default)]
pub struct InfoOmfCommand;

impl GenericCommand for InfoOmfCommand {
  fn name(&self) -> &'static str {
    "info-omf"
  }

  /// Create command for printing omf file info.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to print information about provided omf file")
      .arg(
        Arg::new("path")
          .help("Path to ogf file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Print information about ogf file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    println!("Read omf file {}", path.display());

    let omf_file: Box<OmfFile> = Box::new(OmfFile::read_from_path::<XRayByteOrder, &Path>(path)?);

    println!("Omf file information");

    println!("Version: {}", omf_file.parameters.version);

    println!(
      "Motions: {} {}",
      omf_file.motions.motions.len(),
      omf_file
        .motions
        .motions
        .iter()
        .map(|it| it.name.as_str())
        .collect::<Vec<_>>()
        .join(",")
    );

    println!("Bones total: {}", omf_file.parameters.get_bones_count());
    println!(
      "Parts: {}",
      omf_file
        .parameters
        .parts
        .iter()
        .map(|it| it.name.as_str())
        .collect::<Vec<_>>()
        .join(",")
    );

    for part in &omf_file.parameters.parts {
      println!("Part '{}' bones: {}", part.name, part.get_bones().join(","))
    }

    Ok(())
  }
}

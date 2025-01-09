use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{DatabaseResult, OmfFile, XRayByteOrder};

pub struct InfoOmfCommand {}

impl InfoOmfCommand {
  pub const NAME: &'static str = "info-omf";

  /// Create command for printing omf file info.
  pub fn init() -> Command {
    Command::new(Self::NAME)
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
  pub fn execute(matches: &ArgMatches) -> DatabaseResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    println!("Read omf file {:?}", path);

    let omf_file: OmfFile = OmfFile::read_from_path::<XRayByteOrder>(path)?;

    println!("Omf file information");

    println!("Version: {}", omf_file.parameters.version);

    println!(
      "Motions: {} {:?}",
      omf_file.motions.motions.len(),
      omf_file
        .motions
        .motions
        .iter()
        .map(|it| &it.name)
        .collect::<Vec<&String>>()
    );

    println!("Bones total: {}", omf_file.parameters.get_bones_count());
    println!(
      "Parts: {:?}",
      omf_file
        .parameters
        .parts
        .iter()
        .map(|it| &it.name)
        .collect::<Vec<&String>>()
    );

    for part in &omf_file.parameters.parts {
      println!(
        "Part '{}' bones: {}",
        part.name,
        part
          .bones
          .iter()
          .map(|it| it.0.clone())
          .collect::<Vec<String>>()
          .join(",")
      )
    }

    Ok(())
  }
}

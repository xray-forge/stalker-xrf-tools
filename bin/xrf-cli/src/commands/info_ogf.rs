use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{DatabaseResult, OgfByteOrder, OgfFile};

pub struct InfoOgfCommand {}

impl InfoOgfCommand {
  pub const NAME: &'static str = "info-ogf";

  /// Create command for printing ogf file info.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to print information about provided ogf file")
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

    println!("Read OGF file {:?}", path);

    let ogf_file: OgfFile = OgfFile::read_from_path::<OgfByteOrder>(path)?;

    println!("Ogf file information:");

    println!(
      "Version: {}, model_type: {}, shader_id: {}, {:?} - {:?}",
      ogf_file.header.version,
      ogf_file.header.model_type,
      ogf_file.header.shader_id,
      ogf_file.header.bounding_box,
      ogf_file.header.bounding_sphere
    );

    println!("Boundaries box: {:?}", ogf_file.header.bounding_box,);
    println!("Boundaries sphere: {:?}", ogf_file.header.bounding_sphere);

    if let Some(description) = ogf_file.description {
      println!("Description: {:?}", description);
    }

    Ok(())
  }
}

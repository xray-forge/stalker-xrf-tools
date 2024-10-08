use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xray_icon::{pack_xml_descriptions, ImageFormat, PackDescriptionOptions};

pub struct PackTextureDescriptionCommand {}

impl PackTextureDescriptionCommand {
  pub const NAME: &'static str = "pack-texture-description";

  /// Create command for packing of texture description file.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to pack texture description xml")
      .arg(
        Arg::new("description")
          .help("Path to XML file describing textures")
          .long("description")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("base")
          .help("Path to base where search for described files")
          .long("base")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("output")
          .help("Path to directory where output dds files")
          .long("output")
          .required(false)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("verbose")
          .help("Turn on verbose logging")
          .short('v')
          .long("verbose")
          .required(false)
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("strict")
          .help("Turn on strict unpack mode")
          .short('s')
          .long("strict")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Pack texture descriptions file as single dds sprite.
  pub fn execute(matches: &ArgMatches) {
    let description: &PathBuf = matches
      .get_one::<PathBuf>("description")
      .expect("Expected valid path to be provided for texture description file or folder");

    let base: &PathBuf = matches
      .get_one::<PathBuf>("base")
      .expect("Expected valid base path to be provided");

    let output: &PathBuf = matches.get_one::<PathBuf>("output").unwrap_or(base);

    let is_verbose: bool = matches.get_flag("verbose");
    let is_strict: bool = matches.get_flag("strict");

    let options: PackDescriptionOptions = PackDescriptionOptions {
      description: description.clone(),
      base: base.clone(),
      output: output.clone(),
      dds_compression_format: ImageFormat::BC3RgbaUnorm,
      is_verbose,
      is_strict,
    };

    log::info!("Packing texture descriptions from: {:?}", description);
    log::info!("Paths: base {:?}, output {:?}", base, output);
    log::info!("DDS format: {:?}", options.dds_compression_format);

    pack_xml_descriptions(options)
  }
}

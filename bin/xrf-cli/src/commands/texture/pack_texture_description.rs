use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xray_texture::{ImageFormat, PackDescriptionOptions, PackDescriptionProcessor};

#[derive(Default)]
pub struct PackTextureDescriptionCommand;

impl GenericCommand for PackTextureDescriptionCommand {
  fn name(&self) -> &'static str {
    "pack-texture-description"
  }

  /// Create command for packing of texture description file.
  fn init(&self) -> Command {
    Command::new(self.name())
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
      .arg(
        Arg::new("parallel")
          .help("Turn on parallel mode for pack operations")
          .long("parallel")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Pack texture descriptions file as single dds sprite.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let description: &PathBuf = matches
      .get_one::<PathBuf>("description")
      .expect("Expected valid path to be provided for texture description file or folder");

    let base: &PathBuf = matches
      .get_one::<PathBuf>("base")
      .expect("Expected valid base path to be provided");

    let output: &PathBuf = matches.get_one::<PathBuf>("output").unwrap_or(base);

    let is_verbose: bool = matches.get_flag("verbose");
    let is_strict: bool = matches.get_flag("strict");
    let is_parallel: bool = matches.get_flag("parallel");

    let options: PackDescriptionOptions = PackDescriptionOptions {
      description: description.clone(),
      base: base.clone(),
      output: output.clone(),
      dds_compression_format: ImageFormat::BC3RgbaUnorm,
      is_verbose,
      is_strict,
      is_parallel,
    };

    log::info!("Packing texture descriptions from: {:?}", description);
    log::info!("Paths: base {:?}, output {:?}", base, output);
    log::info!("DDS format: {:?}", options.dds_compression_format);

    PackDescriptionProcessor::pack_xml_descriptions(&options)?;

    Ok(())
  }
}

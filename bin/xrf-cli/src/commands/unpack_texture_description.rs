use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::time::Instant;
use xray_texture::{
  ImageFormat, PackDescriptionOptions, TextureResult, UnpackDescriptionProcessor,
};

pub struct UnpackTextureDescriptionCommand {}

impl UnpackTextureDescriptionCommand {
  pub const NAME: &'static str = "unpack-texture-description";

  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to unpack dds icons into multiple icons")
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
          .help("Path to output folder for icons")
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
          .help("Turn on parallel unpack mode")
          .long("parallel")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  pub fn execute(matches: &ArgMatches) -> TextureResult {
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

    let started_at: Instant = Instant::now();

    log::info!("Unpacking texture descriptions from: {:?}", description);
    log::info!("Paths: base {:?}, output {:?}", base, output);
    log::info!("Parallel mode: {is_parallel}");

    println!(
      "Unpacking texture descriptions: {:?}, from {:?} to {:?}, parallel - {is_parallel}",
      description, base, output
    );

    UnpackDescriptionProcessor::unpack_xml_descriptions(PackDescriptionOptions {
      description: description.clone(),
      base: base.clone(),
      output: output.clone(),
      dds_compression_format: ImageFormat::BC3RgbaUnorm,
      is_verbose,
      is_strict,
      is_parallel,
    })?;

    log::info!(
      "Unpack texture descriptions took: {:?}ms",
      started_at.elapsed().as_millis()
    );

    Ok(())
  }
}

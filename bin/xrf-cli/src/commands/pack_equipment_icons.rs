use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::process;
use xray_icon::{pack_equipment_icons_by_ltx, ImageFormat, PackEquipmentOptions, TextureResult};
use xray_ltx::Ltx;

pub struct PackEquipmentIconsCommand {}

impl PackEquipmentIconsCommand {
  pub const NAME: &'static str = "pack-equipment-icons";

  /// Create command for packing equipment icons.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to pack dds icons into single element")
      .arg(
        Arg::new("system-ltx")
          .help("Path to system ltx file or root folder with ltx files")
          .long("system-ltx")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("source")
          .help("Path to source folder with section icons")
          .long("source")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("output")
          .help("Path to output dds file")
          .long("output")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("gamedata")
          .help("Path to gamedata folder for resources usage")
          .long("gamedata")
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
          .help("Turn on strict mode")
          .short('s')
          .long("strict")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Command to pack equipment icons files into single dds file.
  pub fn execute(matches: &ArgMatches) -> TextureResult {
    let system_ltx_path: &PathBuf = matches
      .get_one::<PathBuf>("system-ltx")
      .expect("Expected valid path to be provided for system-ltx");

    let source: &PathBuf = matches
      .get_one::<PathBuf>("source")
      .expect("Expected valid source path to be provided");

    let gamedata: Option<&PathBuf> = matches.get_one::<PathBuf>("gamedata");

    let output: &PathBuf = matches
      .get_one::<PathBuf>("output")
      .expect("Expected valid output path to be provided");

    let is_verbose: bool = matches.get_flag("verbose");
    let is_strict: bool = matches.get_flag("strict");

    if !source.is_dir() {
      println!("Expected valid source folder containing DDS icons");
      process::exit(1);
    }

    println!("Starting packing DDS icons file");
    println!("System ltx: {:?}", system_ltx_path);
    println!("Source icons dir: {:?}", source);

    let system_ltx: Ltx = Ltx::load_from_file_full(system_ltx_path)?;

    let options = PackEquipmentOptions {
      ltx: system_ltx,
      source: source.into(),
      output: output.into(),
      gamedata: gamedata.cloned(),
      dds_compression_format: ImageFormat::BC3RgbaUnorm,
      is_verbose,
      is_strict,
    };

    log::info!("DDS format: {:?}", options.dds_compression_format);

    pack_equipment_icons_by_ltx(options)?;

    println!("Saved resulting file with combined icons {:?}", output);

    Ok(())
  }
}

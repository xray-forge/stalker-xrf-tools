use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::time::Instant;
use xray_ltx::Ltx;
use xray_texture::{
  dds_to_image, read_dds_by_path, ImageFormat, RgbaImage, UnpackEquipmentOptions,
  UnpackEquipmentProcessor,
};

#[derive(Default)]
pub struct UnpackEquipmentIconsCommand;

impl GenericCommand for UnpackEquipmentIconsCommand {
  fn name(&self) -> &'static str {
    "unpack-equipment-icons"
  }

  /// Create command to unpack equipment icons.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to unpack dds icons into multiple icons")
      .arg(
        Arg::new("system-ltx")
          .help("Path to system ltx file or root folder with ltx files")
          .long("system-ltx")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("source")
          .help("Path to source dds file")
          .long("source")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("output")
          .help("Path to output folder for sections icons")
          .long("output")
          .required(true)
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
  }

  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let system_ltx_path: &PathBuf = matches
      .get_one::<PathBuf>("system-ltx")
      .expect("Expected valid path to be provided for system-ltx");

    let source: &PathBuf = matches
      .get_one::<PathBuf>("source")
      .expect("Expected valid source path to be provided");

    let output: &PathBuf = matches
      .get_one::<PathBuf>("output")
      .expect("Expected valid output folder path to be provided");

    let is_verbose: bool = matches.get_flag("verbose");

    let started_at: Instant = Instant::now();

    println!("Opening DDS file: {:?}", source);

    let source_dds: RgbaImage = read_dds_by_path(source)
      .and_then(|dds| {
        println!(
          "Source DDS file details: {}x{}, mip-maps: {:?}, format: {:?}",
          dds.header.width,
          dds.header.height,
          dds.header.mip_map_count.unwrap_or(0),
          dds.header10.as_ref().map(|header| header.dxgi_format)
        );

        dds_to_image(&dds)
      })
      .expect("Expected path to valid DDS source file");
    let system_ltx: Ltx = Ltx::load_from_file_full(system_ltx_path)?;

    println!("Unpacking equipment DDS file into: {:?}", output);

    create_dir_all(output)?;

    UnpackEquipmentProcessor::unpack_sprites(UnpackEquipmentOptions {
      ltx: system_ltx,
      source: source_dds,
      output: output.into(),
      dds_compression_format: ImageFormat::BC3RgbaUnorm,
      is_verbose,
    })?;

    println!("Successfully DDS equipment file based on LTX sections");

    log::info!(
      "Unpack equipment took: {:?}ms",
      started_at.elapsed().as_millis()
    );

    Ok(())
  }
}

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::fs::create_dir_all;
use std::path::PathBuf;
use xray_icon::{
  dds_to_image, read_dds_by_path, unpack_equipment_icons_by_ltx, ImageFormat, RgbaImage,
  UnpackEquipmentOptions,
};
use xray_ltx::Ltx;

/// Add command to unpack equipment icons.
pub fn add_unpack_equipment_icons_command(command: Command) -> Command {
  command.subcommand(
    Command::new("unpack-equipment-icons")
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
      ),
  )
}

pub fn unpack_equipment_icons(matches: &ArgMatches) {
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
  let system_ltx: Ltx = Ltx::load_from_file_full(system_ltx_path).unwrap();

  println!("Unpacking equipment DDS file into: {:?}", output);

  create_dir_all(output).unwrap();

  unpack_equipment_icons_by_ltx(UnpackEquipmentOptions {
    ltx: system_ltx,
    source: source_dds,
    output: output.into(),
    dds_compression_format: ImageFormat::BC3RgbaUnorm,
    is_verbose,
  });

  println!("Successfully DDS equipment file based on LTX sections");
}

use clap::ArgMatches;
use std::fs::create_dir_all;
use std::path::PathBuf;
use xray_icon::{read_dds_by_path, unpack_equipment_icons_by_ltx, RgbaImage, UnpackOptions};
use xray_ltx::Ltx;

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

  let source_dds: RgbaImage =
    read_dds_by_path(source).expect("Expected path to valid DDS source file");
  let system_ltx: Ltx = Ltx::load_from_file_full(system_ltx_path).unwrap();

  println!(
    "Source file size: {} x {}",
    source_dds.width(),
    source_dds.height()
  );

  println!("Unpacking equipment DDS file into: {:?}", output);

  create_dir_all(output).unwrap();

  unpack_equipment_icons_by_ltx(UnpackOptions {
    ltx: system_ltx,
    source: source_dds,
    output: output.into(),
    is_verbose,
  });

  println!("Successfully DDS equipment file based on LTX sections");
}

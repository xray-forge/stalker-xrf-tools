use clap::ArgMatches;
use std::fs::create_dir_all;
use std::path::PathBuf;
use xray_icon::{read_dds, unpack_ltx, RgbaImage};
use xray_ltx::Ltx;

pub fn unpack_icons(matches: &ArgMatches) {
  let system_ltx_path: &PathBuf = matches
    .get_one::<PathBuf>("system-ltx")
    .expect("Expected valid path to be provided for system-ltx");

  let source: &PathBuf = matches
    .get_one::<PathBuf>("source")
    .expect("Expected valid source path to be provided");

  let output: &PathBuf = matches
    .get_one::<PathBuf>("output")
    .expect("Expected valid output folder path to be provided");

  log::info!("Starting unpacking DDS icons file");

  let source_dds: RgbaImage = read_dds(source);
  let system_ltx: Ltx = Ltx::load_from_file_full(system_ltx_path).unwrap();

  log::info!(
    "Source file size: {} x {}",
    source_dds.width(),
    source_dds.height()
  );

  create_dir_all(output).unwrap();

  unpack_ltx(output, &system_ltx, &source_dds);

  log::info!("Unpack DDS file based on LTX sections");
}

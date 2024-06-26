use clap::ArgMatches;
use std::path::PathBuf;
use std::process;
use xray_icon::{pack_equipment_icons_by_ltx, ImageFormat, PackEquipmentOptions};
use xray_ltx::Ltx;

pub fn pack_equipment_icons(matches: &ArgMatches) {
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

  let system_ltx: Ltx = Ltx::load_from_file_full(system_ltx_path).unwrap();

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

  pack_equipment_icons_by_ltx(options);

  println!("Saved resulting file with combined icons {:?}", output);
}

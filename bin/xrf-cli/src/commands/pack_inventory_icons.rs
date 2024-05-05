use clap::ArgMatches;
use std::path::PathBuf;
use xray_icon::pack_ltx;
use xray_ltx::Ltx;

pub fn pack_icons(matches: &ArgMatches) {
  let system_ltx_path: &PathBuf = matches
    .get_one::<PathBuf>("system-ltx")
    .expect("Expected valid path to be provided for system-ltx");

  let source: &PathBuf = matches
    .get_one::<PathBuf>("source")
    .expect("Expected valid source path to be provided");

  let output: &PathBuf = matches
    .get_one::<PathBuf>("output")
    .expect("Expected valid output path to be provided");

  log::info!("Starting packing DDS icons file");
  log::info!("System ltx: {:?}", system_ltx_path);
  log::info!("Source icons: {:?}", source);
  log::info!("Resulting file: {:?}", output);

  let system_ltx: Ltx = Ltx::load_from_file_full(system_ltx_path).unwrap();

  pack_ltx(&system_ltx, source, output);

  log::info!("Saved resulting file with combined icons");
}

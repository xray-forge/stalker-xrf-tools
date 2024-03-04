use clap::ArgMatches;
use std::path::PathBuf;
use xray_ltx::Ltx;

/// Lint ltx file or folder based on provided arguments.
pub fn format_ltx(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  log::info!("Starting format {:?}", path);

  let ltx: Ltx = Ltx::load_from_file(path).unwrap();

  log::info!("Read ini file: {}", ltx.len())
}

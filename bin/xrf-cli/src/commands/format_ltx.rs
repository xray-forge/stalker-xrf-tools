use clap::ArgMatches;
use std::path::PathBuf;
use xray_ltx::Ini;

/// Lint ltx file or folder based on provided arguments.
pub fn format_ltx(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  log::info!("Starting format {:?}", path);

  let ini: Ini = Ini::load_from_file(path).unwrap();

  log::info!("Read ini file: {}", ini.len())
}

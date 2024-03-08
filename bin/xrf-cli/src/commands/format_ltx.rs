use clap::ArgMatches;
use std::path::PathBuf;
use xray_ltx::{Ltx, LtxProject};

/// Lint and format ltx file or folder based on provided arguments.
pub fn format_ltx(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  if path.is_dir() {
    log::info!("Formatting ltx folder: {:?}", path);
    LtxProject::open_at_path(path)
      .unwrap()
      .format_all_files()
      .unwrap();
  } else {
    log::info!("Formatting ltx file: {:?}", path);
    Ltx::format_file(path).unwrap()
  }
}

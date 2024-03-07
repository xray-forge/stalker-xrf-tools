use clap::ArgMatches;
use std::path::{Path, PathBuf};
use xray_ltx::LtxProject;

/// Verify ltx file or folder based on provided arguments.
pub fn verify_ltx(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  if path.is_dir() {
    log::info!("Verifying ltx folder: {:?}", path);

    LtxProject::open_at_path(path)
      .unwrap()
      .verify_entries()
      .unwrap();
  } else {
    verify_single_ltx(path)
  }
}

fn verify_single_ltx(path: &Path) {
  log::info!("Verifying ltx file: {:?}", path);
}

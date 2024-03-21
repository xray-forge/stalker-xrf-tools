use clap::ArgMatches;
use std::path::PathBuf;
use std::process;
use xray_ltx::{LtxProject, LtxProjectOptions, LtxProjectVerifyResult, LtxVerifyOptions};

/// Verify ltx file or folder based on provided arguments.
pub fn verify_ltx(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  let is_silent: bool = matches.get_flag("silent");
  let is_verbose: bool = matches.get_flag("verbose");
  let is_strict: bool = matches.get_flag("strict");

  if !path.is_dir() {
    println!("Expected configs root directory path for validation as --path parameter");
    process::exit(1);
  }

  log::info!("Verifying ltx folder: {:?}", path);

  let project: LtxProject = LtxProject::open_at_path_opt(
    path,
    LtxProjectOptions {
      is_with_schemes_check: true,
    },
  )
  .unwrap();

  let result: LtxProjectVerifyResult = project
    .verify_entries_opt(LtxVerifyOptions {
      is_silent,
      is_verbose,
      is_strict,
    })
    .unwrap();

  if !result.errors.is_empty() {
    process::exit(1);
  }
}

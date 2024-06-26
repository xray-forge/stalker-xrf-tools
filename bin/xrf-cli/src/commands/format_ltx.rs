use clap::ArgMatches;
use std::path::PathBuf;
use std::process;
use xray_ltx::{Ltx, LtxFormatOptions, LtxProject, LtxProjectFormatResult};

/// Lint and format ltx file or folder based on provided arguments.
pub fn format_ltx(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  let is_silent: bool = matches.get_flag("silent");
  let is_check: bool = matches.get_flag("check");

  if path.is_dir() {
    let project: LtxProject = LtxProject::open_at_path(path)
      .map_err(|error| {
        println!(
          "Failed to format project at {:?}, reason: {:?}",
          path, error
        );

        process::exit(1);
      })
      .unwrap();

    if is_check {
      log::info!("Checking format of ltx folder: {:?}", path);

      let result: LtxProjectFormatResult = project
        .check_format_all_files_opt(LtxFormatOptions { is_silent })
        .unwrap();

      if result.invalid_files > 0 {
        process::exit(1);
      }
    } else {
      log::info!("Formatting ltx folder: {:?}", path);

      project
        .format_all_files_opt(LtxFormatOptions { is_silent })
        .unwrap();
    }
  } else {
    log::info!(
      "Formatting ltx file: {:?}, --check={is_check}, --silent={is_silent}",
      path
    );

    match Ltx::format_file(path, true) {
      Ok(_) => {
        println!("Successfully formatted ltx in '{:?}'", path);
      }
      Err(error) => {
        println!("Failed to format {:?}, reason: {:?}", path, error);
        process::exit(1);
      }
    }
  }
}

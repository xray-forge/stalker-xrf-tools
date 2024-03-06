use clap::ArgMatches;
use std::path::{Path, PathBuf};
use xray_ltx::{EscapePolicy, Ltx, LtxError, LtxProject, ParseOptions, WriteOptions};

/// Verify ltx file or folder based on provided arguments.
pub fn verify_ltx(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  if path.is_dir() {
    verify_project_ltx(path)
  } else {
    verify_single_ltx(path)
  }
}

fn verify_project_ltx(path: &Path) {
  log::info!("Verifying ltx folder: {:?}", path);

  LtxProject::on_root(path).unwrap().verify_entries().unwrap();
}

fn verify_single_ltx(path: &Path) {
  log::info!("Verifying ltx file: {:?}", path);

  let ltx: Result<Ltx, LtxError> = Ltx::load_from_file_opt(
    path,
    ParseOptions {
      enabled_escape: false,
      enabled_quote: false,
    },
  );

  if ltx.is_err() {
    log::error!("Config file is invalid: {}", ltx.unwrap_err().to_string());

    return;
  }

  let ltx: Ltx = ltx
    .unwrap()
    .into_included_opt(ParseOptions {
      enabled_escape: false,
      enabled_quote: false,
    })
    .unwrap()
    .into_inherited()
    .unwrap();

  log::info!("Read ini file: {} {:?}", ltx.len(), ltx.get_directory());

  ltx
    .write_to_file_opt(
      "target/assets/test.ltx",
      WriteOptions {
        escape_policy: EscapePolicy::Nothing,
        ..Default::default()
      },
    )
    .unwrap();
}

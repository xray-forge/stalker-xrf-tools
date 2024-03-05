use clap::ArgMatches;
use std::path::PathBuf;
use xray_ltx::{EscapePolicy, Ltx, ParseOption, WriteOption};

/// Lint ltx file or folder based on provided arguments.
pub fn format_ltx(matches: &ArgMatches) {
  let path: &PathBuf = matches
    .get_one::<PathBuf>("path")
    .expect("Expected valid input path to be provided");

  log::info!("Formatting ltx: {:?}", path);

  let ltx = Ltx::load_from_file_opt(
    path,
    ParseOption {
      enabled_escape: false,
      enabled_quote: false,
    },
  );

  if ltx.is_err() {
    log::error!("Config file is invalid: {}", ltx.unwrap_err().to_string());

    return;
  }

  let ltx: Ltx = ltx.unwrap();

  log::info!("Read ini file: {}", ltx.len());

  ltx
    .write_to_file_opt(
      "test.ltx",
      WriteOption {
        escape_policy: EscapePolicy::Nothing,
        ..Default::default()
      },
    )
    .unwrap();
}

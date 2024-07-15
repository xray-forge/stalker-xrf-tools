use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::process;
use xray_ltx::{LtxProject, LtxProjectOptions, LtxProjectVerifyResult, LtxVerifyOptions};

/// Add command for verifying of ltx files.
pub fn create_verify_ltx_command() -> Command {
  Command::new("verify-ltx")
    .about("Command to verification ltx and ini files")
    .arg(
      Arg::new("path")
        .help("Path to ltx file or folder with ltx files")
        .short('p')
        .long("path")
        .required(true)
        .value_parser(value_parser!(PathBuf)),
    )
    .arg(
      Arg::new("silent")
        .help("Turn of formatter logging")
        .long("silent")
        .required(false)
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("verbose")
        .help("Turn on verbose logging")
        .short('v')
        .long("verbose")
        .required(false)
        .action(ArgAction::SetTrue),
    )
    .arg(
      Arg::new("strict")
        .help("Turn on strict checking mode")
        .short('s')
        .long("strict")
        .required(false)
        .action(ArgAction::SetTrue),
    )
}

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

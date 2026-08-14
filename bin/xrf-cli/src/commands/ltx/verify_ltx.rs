use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use xrf_error::XrfError;
use xrf_ltx::{LtxProject, LtxProjectOptions, LtxProjectVerifyResult, LtxVerifyOptions};
use xrf_output::OutputOptions;

use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;

#[derive(Default)]
pub struct VerifyLtxCommand;

impl GenericCommand for VerifyLtxCommand {
  fn name(&self) -> &'static str {
    "verify-ltx"
  }

  /// Add command for verifying of ltx files.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command for verification of ltx and ini files")
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
          .help("Turn off logging")
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
  }

  /// Verify ltx file or folder based on provided arguments.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid input path to be provided");

    let output: OutputOptions = TerminalOutput::from_options(matches.get_flag("silent"), matches.get_flag("verbose"));

    if !path.is_dir() {
      xrf_output::error!(
        output,
        "Expected configs root directory path for validation as --path parameter"
      );

      return Err(XrfError::new_read_error("Failed to read provided path as directory").into());
    }

    log::info!("Verifying ltx folder: {}", path.display());

    let project: Box<LtxProject> = Box::new(LtxProject::open_at_path_opt(
      path,
      LtxProjectOptions {
        is_with_schemes_check: true,
        is_strict_check: true,
      },
    )?);

    let result: LtxProjectVerifyResult = project.verify_entries_opt(LtxVerifyOptions { output })?;

    if result.errors.is_empty() {
      Ok(())
    } else {
      Err(
        XrfError::new_verify_error(format!(
          "Failed to verify ltx files, got {} errors",
          result.errors.len()
        ))
        .into(),
      )
    }
  }
}

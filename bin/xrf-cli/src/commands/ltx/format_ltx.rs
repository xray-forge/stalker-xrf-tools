use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xray_ltx::{Ltx, LtxError, LtxFormatOptions, LtxProject, LtxProjectFormatResult};

#[derive(Default)]
pub struct FormatLtxCommand;

impl GenericCommand for FormatLtxCommand {
  fn name(&self) -> &'static str {
    "format-ltx"
  }

  /// Create command for verifying of spawn file.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to format ltx and ini files")
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
        Arg::new("check")
          .help("Turn of formatter logging")
          .short('c')
          .long("check")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Lint and format ltx file or folder based on provided arguments.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid input path to be provided");

    let is_silent: bool = matches.get_flag("silent");
    let is_check: bool = matches.get_flag("check");

    if path.is_dir() {
      let project: LtxProject = LtxProject::open_at_path(path).map_err(|error| {
        println!(
          "Failed to format project at {:?}, reason: {:?}",
          path, error
        );

        error
      })?;

      if is_check {
        log::info!("Checking format of ltx folder: {:?}", path);

        let result: LtxProjectFormatResult =
          project.check_format_all_files_opt(LtxFormatOptions { is_silent })?;

        if result.invalid_files > 0 {
          return Err(
            LtxError::new_verify_error("Project includes LTX files with invalid format").into(),
          );
        }
      } else {
        log::info!("Formatting ltx folder: {:?}", path);

        project.format_all_files_opt(LtxFormatOptions { is_silent })?;
      }

      Ok(())
    } else {
      log::info!(
        "Formatting ltx file: {:?}, --check={is_check}, --silent={is_silent}",
        path
      );

      // todo: Check single file?
      match Ltx::format_file(path, true) {
        Ok(_) => {
          println!("Successfully formatted ltx in '{:?}'", path);

          Ok(())
        }
        Err(error) => {
          println!("Failed to format {:?}, reason: {:?}", path, error);

          Err(Box::new(error))
        }
      }
    }
  }
}

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use walkdir::{DirEntry, WalkDir};
use xray_error::XRayError;
use xray_ltx::{LTX_EXTENSION, LtxFilesFormatter, LtxFormatOptions, LtxProjectFormatResult};
use xray_output::OutputOptions;

use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;

#[derive(Default)]
pub struct FormatLtxCommand;

impl GenericCommand for FormatLtxCommand {
  fn name(&self) -> &'static str {
    "format-ltx"
  }

  /// Create command for formatting of ltx files.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to format ltx and ini files")
      .arg(
        Arg::new("path")
          .help("Paths to ltx files or folders with ltx files")
          .short('p')
          .long("path")
          .required(true)
          .num_args(1..)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("check")
          .help("Run formatter in check mode")
          .short('c')
          .long("check")
          .required(false)
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("silent")
          .help("Turn off logging")
          .long("silent")
          .short('s')
          .required(false)
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("verbose")
          .help("Turn on verbose logging")
          .long("verbose")
          .short('v')
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Format ltx files or folders based on provided arguments.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let paths: Vec<&PathBuf> = matches
      .get_many::<PathBuf>("path")
      .expect("Expected valid input paths to be provided")
      .collect();

    let is_check: bool = matches.get_flag("check");

    let output: OutputOptions = TerminalOutput::from_options(matches.get_flag("silent"), matches.get_flag("verbose"));

    let files: Vec<PathBuf> = Self::collect_ltx_files(&paths)?;

    log::info!(
      "{} {} ltx file(s) from {} provided path(s)",
      if is_check { "Checking" } else { "Formatting" },
      files.len(),
      paths.len()
    );

    let options: LtxFormatOptions = LtxFormatOptions { output: output.clone() };

    if is_check {
      let result: LtxProjectFormatResult = LtxFilesFormatter::check_format_opt(&files, options)?;

      if result.invalid_files > 0 {
        return Err(XRayError::new_verify_error("Project includes LTX files with invalid format").into());
      }
    } else {
      LtxFilesFormatter::format_opt(&files, options)?;
    }

    Ok(())
  }
}

impl FormatLtxCommand {
  /// Expand provided paths into a de-duplicated list of ltx files.
  ///
  /// Folders are walked recursively for `*.ltx` entries, while explicitly provided files are taken
  /// as is - so callers can format an arbitrary subset without matching the folder extension rules.
  fn collect_ltx_files(paths: &[&PathBuf]) -> Result<Vec<PathBuf>, XRayError> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut visited: HashSet<PathBuf> = HashSet::new();

    for path in paths {
      if path.is_dir() {
        for entry in WalkDir::new(path) {
          let entry: DirEntry = entry.map_err(|error| error.into_io_error().unwrap())?;
          let entry_path: &Path = entry.path();

          if entry_path.is_file()
            && entry_path
              .extension()
              .is_some_and(|extension| extension == LTX_EXTENSION)
            && visited.insert(entry_path.into())
          {
            files.push(entry_path.into());
          }
        }
      } else if path.exists() {
        if visited.insert((*path).clone()) {
          files.push((*path).clone());
        }
      } else {
        return Err(XRayError::new_not_found_error(format!(
          "Failed to format ltx, provided path does not exist: {}",
          path.display()
        )));
      }
    }

    Ok(files)
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;

  use xray_error::XRayResult;

  use super::FormatLtxCommand;
  use crate::generic_command::{CommandResult, GenericCommand};

  fn create_root(name: &str) -> XRayResult<PathBuf> {
    let root: PathBuf = std::env::temp_dir().join(format!("xrf-cli-format-ltx-{name}-{}", std::process::id()));

    if root.exists() {
      fs::remove_dir_all(&root)?;
    }

    fs::create_dir_all(&root)?;

    Ok(root)
  }

  #[test]
  fn collects_ltx_files_from_folder_recursively() -> XRayResult {
    let root: PathBuf = create_root("folder")?;
    let nested: PathBuf = root.join("nested");

    fs::create_dir_all(&nested)?;
    fs::write(root.join("first.ltx"), "[a]\n")?;
    fs::write(nested.join("second.ltx"), "[b]\n")?;
    fs::write(root.join("ignored.txt"), "text")?;

    let mut files: Vec<PathBuf> = FormatLtxCommand::collect_ltx_files(&[&root])?;

    files.sort();

    assert_eq!(files, vec![root.join("first.ltx"), nested.join("second.ltx")]);

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn collects_explicitly_provided_files_regardless_of_extension() -> XRayResult {
    let root: PathBuf = create_root("explicit")?;
    let ltx: PathBuf = root.join("first.ltx");
    let ini: PathBuf = root.join("second.ini");

    fs::write(&ltx, "[a]\n")?;
    fs::write(&ini, "[b]\n")?;

    let files: Vec<PathBuf> = FormatLtxCommand::collect_ltx_files(&[&ltx, &ini])?;

    assert_eq!(files, vec![ltx, ini]);

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn de_duplicates_mixed_folder_and_file_paths() -> XRayResult {
    let root: PathBuf = create_root("mixed")?;
    let first: PathBuf = root.join("first.ltx");

    fs::write(&first, "[a]\n")?;
    fs::write(root.join("second.ltx"), "[b]\n")?;

    let mut files: Vec<PathBuf> = FormatLtxCommand::collect_ltx_files(&[&root, &first])?;

    files.sort();

    assert_eq!(files, vec![first, root.join("second.ltx")]);

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn fails_on_missing_path() -> XRayResult {
    let root: PathBuf = create_root("missing")?;
    let missing: PathBuf = root.join("absent.ltx");

    assert!(FormatLtxCommand::collect_ltx_files(&[&missing]).is_err());

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn preserves_standalone_semicolon_comments() -> CommandResult {
    let root: PathBuf = create_root("standalone-comment")?;
    let file: PathBuf = root.join("comment.ltx");

    fs::write(&file, ";\n")?;

    let command: FormatLtxCommand = FormatLtxCommand;
    let matches =
      command
        .init()
        .try_get_matches_from(["format-ltx", "--path", &file.display().to_string(), "--silent"])?;

    command.execute(&matches)?;

    assert_eq!(fs::read_to_string(&file)?, ";\r\n");

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn formatting_standalone_comment_is_idempotent() -> CommandResult {
    let root: PathBuf = create_root("idempotent-comment")?;
    let file: PathBuf = root.join("comment.ltx");
    let expected: Vec<u8> = b";\r\n".to_vec();

    fs::write(&file, &expected)?;

    let command: FormatLtxCommand = FormatLtxCommand;
    let matches =
      command
        .init()
        .try_get_matches_from(["format-ltx", "--path", &file.display().to_string(), "--silent"])?;

    command.execute(&matches)?;
    let formatted_once: Vec<u8> = fs::read(&file)?;

    command.execute(&matches)?;
    let formatted_twice: Vec<u8> = fs::read(&file)?;

    assert_eq!(formatted_once, expected);
    assert_eq!(formatted_twice, formatted_once);

    fs::remove_dir_all(root)?;

    Ok(())
  }
}

use std::path::PathBuf;
use std::str::FromStr;

use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use xrf_output::OutputOptions;
use xrf_translation::{ProjectBuildOptions, ProjectBuildResult, TranslationLanguage, build_dir, build_file};

use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;

#[derive(Default)]
pub struct BuildTranslationsCommand;

impl GenericCommand for BuildTranslationsCommand {
  fn name(&self) -> &'static str {
    "build-translation"
  }

  /// Create command for building of translation files.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to build translation files into gamedata")
      .arg(
        Arg::new("path")
          .help("Path to translation folder")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("output")
          .help("Path to output translation")
          .short('o')
          .long("output")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("language")
          .help("Target language to translate")
          .short('l')
          .long("language")
          .required(false)
          .default_value("all")
          .value_parser(value_parser!(String)),
      )
      .arg(
        Arg::new("silent")
          .help("Disable any logging")
          .short('s')
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
        Arg::new("sort")
          .help("Preserve source order instead of sorting dynamic translation files")
          .long("no-sort")
          .required(false)
          .action(ArgAction::SetFalse),
      )
  }

  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let output_dir: &PathBuf = matches
      .get_one::<PathBuf>("output")
      .expect("Expected valid output folder path to be provided");

    let language: &String = matches
      .get_one::<String>("language")
      .expect("Expected valid language for translation");

    let is_silent: bool = matches.get_flag("silent");
    let is_verbose: bool = matches.get_flag("verbose");
    let is_sorted: bool = matches.get_flag("sort");

    let output: OutputOptions = TerminalOutput::from_options(is_silent, is_verbose);

    xrf_output::info!(
      output,
      "Building translation {}, language - {}, sorted - {}",
      path.display(),
      language,
      is_sorted
    );

    let options: ProjectBuildOptions = ProjectBuildOptions {
      is_sorted,
      output,
      path: path.clone(),
      output_dir: output_dir.clone(),
      language: TranslationLanguage::from_str(language)?,
    };

    let result: ProjectBuildResult = if path.is_dir() {
      build_dir(path, &options)?
    } else {
      build_file(path, &options)?
    };

    xrf_output::info!(
      options.output,
      "Built translation files in {} sec",
      (result.duration as f64) / 1000.0
    );

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use clap::ArgMatches;

  use super::BuildTranslationsCommand;
  use crate::generic_command::GenericCommand;

  fn parse_matches(extra: &[&str]) -> ArgMatches {
    let mut arguments = vec!["build-translation", "--path", "translations", "--output", "output"];

    arguments.extend_from_slice(extra);

    BuildTranslationsCommand.init().try_get_matches_from(arguments).unwrap()
  }

  #[test]
  fn translations_are_sorted_unless_source_order_is_requested() {
    assert!(parse_matches(&[]).get_flag("sort"));
    assert!(!parse_matches(&["--no-sort"]).get_flag("sort"));
  }
}

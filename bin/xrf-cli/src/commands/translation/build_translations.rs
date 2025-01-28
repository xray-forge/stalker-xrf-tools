use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::str::FromStr;
use xray_translation::{
  ProjectBuildOptions, ProjectBuildResult, TranslationLanguage, TranslationProject,
};

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
          .help("Turn on sorting for dynamic translation files")
          .long("sort")
          .required(false)
          .action(ArgAction::SetFalse),
      )
  }

  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let output: &PathBuf = matches
      .get_one::<PathBuf>("output")
      .expect("Expected valid output folder path to be provided");

    let language: &String = matches
      .get_one::<String>("language")
      .expect("Expected valid language for translation");

    let is_silent: bool = matches.get_flag("silent");
    let is_verbose: bool = matches.get_flag("verbose");
    let is_sorted: bool = matches.get_flag("sort");

    if !is_silent {
      println!(
        "Building translation {}, language - {}, sorted - {}",
        path.display(),
        language,
        is_sorted
      )
    }

    let options: ProjectBuildOptions = ProjectBuildOptions {
      is_sorted,
      is_silent,
      is_verbose,
      path: path.clone(),
      output: output.clone(),
      language: TranslationLanguage::from_str(language)?,
    };

    let result: ProjectBuildResult = if path.is_dir() {
      TranslationProject::build_dir(path, &options)?
    } else {
      TranslationProject::build_file(path, &options)?
    };

    if options.is_logging_enabled() {
      println!(
        "Built translation files in {} sec",
        (result.duration as f64) / 1000.0
      );
    }

    Ok(())
  }
}

use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use xray_translation::{
  ProjectVerifyOptions, ProjectVerifyResult, TranslationLanguage, TranslationProject,
};

#[derive(Default)]
pub struct VerifyTranslationsCommand;

impl GenericCommand for VerifyTranslationsCommand {
  fn name(&self) -> &'static str {
    "verify-translation"
  }

  /// Create command for verifying of translation files.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to verify translation files integrity")
      .arg(
        Arg::new("path")
          .help("Path to translation folder")
          .short('p')
          .long("path")
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
        Arg::new("strict")
          .help("Fail with non 0 error code if translation are missing")
          .long("strict")
          .required(false)
          .action(ArgAction::SetTrue),
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
  }

  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let language: &String = matches
      .get_one::<String>("language")
      .expect("Expected valid language for translation");

    let is_silent: bool = matches.get_flag("silent");
    let is_verbose: bool = matches.get_flag("verbose");
    let is_strict: bool = matches.get_flag("strict");

    if !is_silent {
      println!("Verifying translation {:?}, language - {language}", path)
    }

    let options: ProjectVerifyOptions = ProjectVerifyOptions {
      is_strict,
      is_silent,
      is_verbose,
      path: path.clone(),
      language: TranslationLanguage::from_str(language)?,
    };

    let result: ProjectVerifyResult = if path.is_dir() {
      TranslationProject::verify_dir(path, &options)?
    } else {
      TranslationProject::verify_file(path, &options)?
    };

    if options.is_logging_enabled() {
      println!(
        "Verified translation files in {} sec, {} checked, {} missing",
        (result.duration as f64) / 1000.0,
        result.checked_translations_count,
        result.missing_translations_count
      );
    }

    if options.is_strict && result.missing_translations_count > 0 {
      log::error!("Failing with non-zero error code, missing translation found");
      process::exit(1);
    }

    Ok(())
  }
}

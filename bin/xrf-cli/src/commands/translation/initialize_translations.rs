use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xray_translation::{ProjectInitializeOptions, ProjectInitializeResult, TranslationProject};

#[derive(Default)]
pub struct InitializeTranslationsCommand;

impl GenericCommand for InitializeTranslationsCommand {
  fn name(&self) -> &'static str {
    "initialize-translation"
  }

  /// Create command for initialization of translation files.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to initialize translation files")
      .arg(
        Arg::new("path")
          .help("Path to translation folder")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
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

    let is_silent: bool = matches.get_flag("silent");
    let is_verbose: bool = matches.get_flag("verbose");

    if !is_silent {
      println!("Verifying translation {}", path.display())
    }

    let options: ProjectInitializeOptions = ProjectInitializeOptions {
      is_silent,
      is_verbose,
      path: path.clone(),
    };

    let result: ProjectInitializeResult = if path.is_dir() {
      TranslationProject::initialize_dir(path, &options)?
    } else {
      TranslationProject::initialize_file(path, &options)?
    };

    if options.is_logging_enabled() {
      println!(
        "Initialized translation files in {} sec",
        (result.duration as f64) / 1000.0,
      );
    }

    Ok(())
  }
}

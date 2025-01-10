use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

#[derive(Default)]
pub struct ParseTranslationsCommand;

impl GenericCommand for ParseTranslationsCommand {
  fn name(&self) -> &'static str {
    "parse-translation"
  }

  /// Create translation parsing command.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to parse xml translation into json variants")
      .arg(
        Arg::new("path")
          .help("Path to translation folder")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Parse translation from path as json.
  fn execute(&self, _matches: &ArgMatches) -> CommandResult {
    // todo;

    Ok(())
  }
}

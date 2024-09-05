use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

pub struct ParseTranslationsCommand {}

impl ParseTranslationsCommand {
  pub const NAME: &'static str = "parse-translations";

  /// Create translations parsing command.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to parse xml translations into json variants")
      .arg(
        Arg::new("path")
          .help("Path to translations folder")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Parse translations from path as json.
  pub fn execute(_matches: &ArgMatches) {
    // todo;
  }
}

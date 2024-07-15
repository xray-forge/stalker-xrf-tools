use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

/// Create translations parsing command.
pub fn create_parse_translations_command() -> Command {
  Command::new("parse-translations")
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
pub fn parse_translations(matches: &ArgMatches) {
  // todo;
}

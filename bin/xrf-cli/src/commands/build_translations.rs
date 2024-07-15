use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

/// Create command for verifying of spawn file.
pub fn create_build_translations_command() -> Command {
  Command::new("build-translations")
    .about("Command to build translation files into gamedata")
    .arg(
      Arg::new("path")
        .help("Path to translations folder")
        .short('p')
        .long("path")
        .required(true)
        .value_parser(value_parser!(PathBuf)),
    )
}

pub fn build_translations(matches: &ArgMatches) {
  // todo;
}

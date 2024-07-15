use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

/// Add command for verifying of spawn file.
pub fn add_build_translations_command(command: Command) -> Command {
  command.subcommand(
    Command::new("build-translations")
      .about("Command to build translation files into gamedata")
      .arg(
        Arg::new("path")
          .help("Path to translations folder")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      ),
  )
}

pub fn build_translations(matches: &ArgMatches) {
  // todo;
}

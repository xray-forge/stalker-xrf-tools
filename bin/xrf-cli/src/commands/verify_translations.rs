use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;

/// Add command for verifying of translation files.
pub fn add_verify_translations_command(command: Command) -> Command {
  command.subcommand(
    Command::new("verify-translations")
      .about("Command to verify translation files integrity")
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

pub fn verify_translations(matches: &ArgMatches) {
  // todo;
}

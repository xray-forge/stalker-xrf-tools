use std::process::ExitCode;

use clap::{ArgMatches, Command};

use crate::core::generic_command::GenericCommand;
use crate::registry::setup_subcommands;

/// Assemble the CLI from the registered commands and run the one the caller asked for.
///
/// The only place a command outcome becomes a process exit. Every failure ends with exactly one
/// final stderr line, printed unconditionally so `--silent` can never hide that a run failed;
/// commands themselves report finding details and never exit.
pub fn run() -> ExitCode {
  let commands: Vec<Box<dyn GenericCommand>> = setup_subcommands();

  let mut application: Command = Command::new("xrf-tool")
    .about("XRF forge CLI tools application")
    .arg_required_else_help(true);

  for command in &commands {
    application = application.subcommand(command.init());
  }

  let matches: ArgMatches = application.get_matches();

  // `arg_required_else_help` already answered the empty invocation, and clap rejects a subcommand it
  // never advertised, so both misses below mean the registry and the parser disagree.
  let Some((name, arguments)) = matches.subcommand() else {
    unreachable!("clap matched no subcommand after requiring one")
  };

  let Some(command) = commands.iter().find(|command| command.name() == name) else {
    unreachable!("clap matched '{name}', which no registered command declares")
  };

  match command.execute(arguments) {
    Ok(()) => ExitCode::SUCCESS,
    Err(error) => {
      eprintln!("{error}");

      ExitCode::from(error.exit_code())
    }
  }
}

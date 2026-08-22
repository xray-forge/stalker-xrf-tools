use std::process;

use clap::{ArgMatches, Command};

use crate::core::generic_command::GenericCommand;
use crate::core::output::TerminalOutput;
use crate::registry::setup_subcommands;

/// Assemble the CLI from the registered commands and run the one the caller asked for.
pub fn run() {
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

  if let Err(error) = command.execute(arguments) {
    xrf_output::error!(
      TerminalOutput::from_options(false, false),
      "Execution of command '{name}' failed, error: {error}",
    );

    process::exit(1);
  }
}

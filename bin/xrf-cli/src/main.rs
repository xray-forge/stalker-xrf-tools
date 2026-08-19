pub(crate) mod commands;
pub(crate) mod generic_command;
pub(crate) mod logger;
pub(crate) mod output;
pub(crate) mod setup;

use std::error::Error;
use std::process;

use clap::Command;

use crate::generic_command::{CommandResult, GenericCommand};
use crate::logger::setup_logger;
use crate::output::TerminalOutput;
use crate::setup::setup_subcommands;

fn main() -> Result<(), Box<dyn Error>> {
  setup_logger();

  let mut command: Command = Command::new("xrf-tool").about("XRF forge CLI tools application");
  let subcommands: Vec<Box<dyn GenericCommand>> = setup_subcommands();

  for subcommand in &subcommands {
    command = command.subcommand(subcommand.init());
  }

  if let Some((command_name, matches)) = command.get_matches().subcommand() {
    subcommands
      .iter()
      .find(|it| it.name() == command_name)
      .map(|it| {
        let result: CommandResult = it.execute(matches);

        if let Err(error) = &result {
          xrf_output::error!(
            TerminalOutput::from_options(false, false),
            "Execution of command '{}' failed, error: {}",
            it.name(),
            error
          );
          process::exit(1);
        } else {
          result
        }
      })
      .expect("Valid subcommand")?;
  } else {
    panic!("Unexpected cli command provided, check --help for details")
  }

  Ok(())
}

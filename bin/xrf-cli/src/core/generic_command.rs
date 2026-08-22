use clap::{ArgMatches, Command};

use crate::core::command_error::CommandError;

pub type CommandResult<T = ()> = Result<T, CommandError>;

/// Named set of related commands; drives both CLI registration order and generated documentation layout.
pub struct CommandGroup {
  pub name: &'static str,
  pub commands: Vec<Box<dyn GenericCommand>>,
}

pub trait GenericCommand {
  fn new() -> Self
  where
    Self: Sized + Default,
  {
    Self::default()
  }

  fn new_box() -> Box<Self>
  where
    Self: Sized + Default,
  {
    Box::new(Self::default())
  }

  fn name(&self) -> &'static str;

  fn init(&self) -> Command;

  fn execute(&self, matches: &ArgMatches) -> CommandResult;
}

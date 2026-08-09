use std::error::Error;

use clap::{ArgMatches, Command};

pub type CommandResult<T = ()> = Result<T, Box<dyn Error>>;

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

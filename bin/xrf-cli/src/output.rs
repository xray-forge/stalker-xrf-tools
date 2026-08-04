use colored::Colorize;
use std::fmt::Display;
use xray_output::Output;

/// Terminal renderer for live workflow output.
#[derive(Default)]
pub struct TerminalOutput;

impl Output for TerminalOutput {
  fn heading(&self, message: &dyn Display) {
    println!("{}", message.to_string().green());
  }

  fn success(&self, message: &dyn Display) {
    println!("{}", message.to_string().green());
  }

  fn warning(&self, message: &dyn Display) {
    eprintln!("{}", message.to_string().yellow());
  }

  fn failure(&self, message: &dyn Display) {
    eprintln!("{}", message.to_string().red());
  }

  fn info(&self, message: &dyn Display) {
    println!("{message}");
  }

  fn error(&self, message: &dyn Display) {
    eprintln!("{message}");
  }

  fn verbose(&self, message: &dyn Display) {
    println!("{message}");
  }
}

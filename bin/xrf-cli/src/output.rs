use colored::Colorize;
use std::fmt::Display;
use std::sync::Arc;
use xray_output::{Output, OutputOptions, OutputVerbosity};

/// Terminal renderer for live workflow output.
#[derive(Default)]
pub struct TerminalOutput;

impl TerminalOutput {
  /// Creates terminal output configured from the CLI verbosity flags.
  pub fn from_options(is_silent: bool, is_verbose: bool) -> OutputOptions {
    let verbosity: OutputVerbosity = match (is_silent, is_verbose) {
      (true, _) => OutputVerbosity::Silent,
      (false, true) => OutputVerbosity::Verbose,
      (false, false) => OutputVerbosity::Normal,
    };

    OutputOptions::new(Arc::new(Self), verbosity)
  }
}

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

#[cfg(test)]
mod tests {
  use super::TerminalOutput;
  use xray_output::OutputVerbosity;

  #[test]
  fn maps_cli_verbosity_flags() {
    assert_eq!(
      TerminalOutput::from_options(true, true).verbosity(),
      OutputVerbosity::Silent
    );
    assert_eq!(
      TerminalOutput::from_options(false, true).verbosity(),
      OutputVerbosity::Verbose
    );
    assert_eq!(
      TerminalOutput::from_options(false, false).verbosity(),
      OutputVerbosity::Normal
    );
  }
}

use std::fmt::Display;
use std::sync::Arc;

use crate::{NoopOutput, Output};

/// Controls which live workflow messages are rendered.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OutputVerbosity {
  #[default]
  Silent,
  Normal,
  Verbose,
}

/// Runtime output configuration for a workflow.
#[derive(Clone)]
pub struct OutputOptions {
  output: Arc<dyn Output>,
  verbosity: OutputVerbosity,
}

impl OutputOptions {
  pub fn new(output: Arc<dyn Output>, verbosity: OutputVerbosity) -> Self {
    Self { output, verbosity }
  }

  pub fn heading(&self, message: impl Display) {
    if self.verbosity != OutputVerbosity::Silent {
      self.output.heading(&message);
    }
  }

  pub fn success(&self, message: impl Display) {
    if self.verbosity != OutputVerbosity::Silent {
      self.output.success(&message);
    }
  }

  pub fn warning(&self, message: impl Display) {
    if self.verbosity != OutputVerbosity::Silent {
      self.output.warning(&message);
    }
  }

  pub fn failure(&self, message: impl Display) {
    if self.verbosity != OutputVerbosity::Silent {
      self.output.failure(&message);
    }
  }

  pub fn info(&self, message: impl Display) {
    if self.verbosity != OutputVerbosity::Silent {
      self.output.info(&message);
    }
  }

  pub fn error(&self, message: impl Display) {
    if self.verbosity != OutputVerbosity::Silent {
      self.output.error(&message);
    }
  }

  pub fn verbose(&self, message: impl Display) {
    if self.verbosity == OutputVerbosity::Verbose {
      self.output.verbose(&message);
    }
  }

  pub const fn verbosity(&self) -> OutputVerbosity {
    self.verbosity
  }
}

impl Default for OutputOptions {
  fn default() -> Self {
    Self::new(Arc::new(NoopOutput), OutputVerbosity::Silent)
  }
}

#[cfg(test)]
mod tests {
  use std::fmt::Display;
  use std::sync::{Arc, Mutex};

  use super::{OutputOptions, OutputVerbosity};
  use crate::Output;

  #[derive(Default)]
  struct RecordingOutput {
    messages: Mutex<Vec<String>>,
  }

  impl Output for RecordingOutput {
    fn heading(&self, message: &dyn Display) {
      self.messages.lock().unwrap().push(format!("heading:{message}"));
    }

    fn success(&self, message: &dyn Display) {
      self.messages.lock().unwrap().push(format!("success:{message}"));
    }

    fn warning(&self, message: &dyn Display) {
      self.messages.lock().unwrap().push(format!("warning:{message}"));
    }

    fn failure(&self, message: &dyn Display) {
      self.messages.lock().unwrap().push(format!("failure:{message}"));
    }

    fn info(&self, message: &dyn Display) {
      self.messages.lock().unwrap().push(format!("info:{message}"));
    }

    fn error(&self, message: &dyn Display) {
      self.messages.lock().unwrap().push(format!("error:{message}"));
    }

    fn verbose(&self, message: &dyn Display) {
      self.messages.lock().unwrap().push(format!("verbose:{message}"));
    }
  }

  #[test]
  fn filters_messages_by_verbosity() {
    let output: Arc<RecordingOutput> = Arc::new(RecordingOutput::default());
    let options: OutputOptions = OutputOptions::new(output.clone(), OutputVerbosity::Normal);

    options.heading("heading");
    options.success("success");
    options.warning("warning");
    options.failure("failure");
    options.info("normal");
    options.error("error");
    options.verbose("verbose");

    assert_eq!(
      *output.messages.lock().unwrap(),
      vec![
        String::from("heading:heading"),
        String::from("success:success"),
        String::from("warning:warning"),
        String::from("failure:failure"),
        String::from("info:normal"),
        String::from("error:error"),
      ]
    );
  }

  #[test]
  fn forwards_every_message_at_verbose_verbosity() {
    let output: Arc<RecordingOutput> = Arc::new(RecordingOutput::default());
    let options: OutputOptions = OutputOptions::new(output.clone(), OutputVerbosity::Verbose);

    options.info("normal");
    options.error("error");
    options.verbose("verbose");

    assert_eq!(
      *output.messages.lock().unwrap(),
      vec![
        String::from("info:normal"),
        String::from("error:error"),
        String::from("verbose:verbose"),
      ]
    );
  }
}

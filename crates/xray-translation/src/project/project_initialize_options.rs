use std::path::PathBuf;

pub struct ProjectInitializeOptions {
  pub is_silent: bool,
  pub is_verbose: bool,
  pub path: PathBuf,
}

impl ProjectInitializeOptions {
  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}

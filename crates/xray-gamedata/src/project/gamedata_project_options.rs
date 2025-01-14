use std::path::PathBuf;

#[derive(Default)]
pub struct GamedataProjectOpenOptions {
  pub roots: Vec<PathBuf>,
  pub ignored: Vec<String>,
  pub configs: PathBuf,
  pub is_verbose: bool,
  pub is_silent: bool,
  pub is_strict: bool,
}

impl GamedataProjectOpenOptions {
  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}

#[derive(Default)]
pub struct GamedataProjectVerifyOptions {
  pub is_verbose: bool,
  pub is_silent: bool,
  pub is_strict: bool,
}

impl GamedataProjectVerifyOptions {
  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}

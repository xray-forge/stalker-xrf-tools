use crate::project::gamedata_verification_type::GamedataVerificationType;
use std::path::PathBuf;

#[derive(Default)]
pub struct GamedataProjectReadOptions {
  pub roots: Vec<PathBuf>,
  pub ignored: Vec<String>,
  pub configs: PathBuf,
  pub is_verbose: bool,
  pub is_silent: bool,
  pub is_strict: bool,
}

impl GamedataProjectReadOptions {
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
  pub checks: Vec<GamedataVerificationType>,
}

impl GamedataProjectVerifyOptions {
  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}

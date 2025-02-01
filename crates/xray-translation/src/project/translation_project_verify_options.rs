use crate::language::TranslationLanguage;
use std::path::PathBuf;

pub struct ProjectVerifyOptions {
  pub is_strict: bool,
  pub is_silent: bool,
  pub is_verbose: bool,
  pub path: PathBuf,
  pub language: TranslationLanguage,
}

impl ProjectVerifyOptions {
  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}

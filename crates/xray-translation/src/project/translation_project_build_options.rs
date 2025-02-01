use crate::language::TranslationLanguage;
use std::path::PathBuf;

pub struct ProjectBuildOptions {
  pub is_silent: bool,
  pub is_sorted: bool,
  pub is_verbose: bool,
  pub path: PathBuf,
  pub output: PathBuf,
  pub language: TranslationLanguage,
}

impl ProjectBuildOptions {
  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}

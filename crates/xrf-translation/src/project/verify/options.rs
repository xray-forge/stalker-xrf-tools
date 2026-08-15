use std::path::PathBuf;

use crate::language::TranslationLanguage;

pub struct ProjectVerifyOptions {
  pub is_strict: bool,
  pub output: xrf_output::OutputOptions,
  pub path: PathBuf,
  pub language: TranslationLanguage,
}

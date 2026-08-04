use crate::language::TranslationLanguage;
use std::path::PathBuf;

pub struct ProjectVerifyOptions {
  pub is_strict: bool,
  pub output: xray_output::OutputOptions,
  pub path: PathBuf,
  pub language: TranslationLanguage,
}

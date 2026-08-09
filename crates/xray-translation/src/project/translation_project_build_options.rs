use std::path::PathBuf;

use crate::language::TranslationLanguage;

pub struct ProjectBuildOptions {
  pub output: xray_output::OutputOptions,
  pub is_sorted: bool,
  pub path: PathBuf,
  pub output_dir: PathBuf,
  pub language: TranslationLanguage,
}

use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LtxProjectFormatResult {
  pub duration: u128,
  pub invalid_files: usize,
  pub to_format: Vec<PathBuf>,
  pub total_files: usize,
  pub valid_files: usize,
}

impl LtxProjectFormatResult {
  pub fn new() -> LtxProjectFormatResult {
    LtxProjectFormatResult {
      duration: 0,
      invalid_files: 0,
      to_format: Vec::new(),
      total_files: 0,
      valid_files: 0,
    }
  }
}

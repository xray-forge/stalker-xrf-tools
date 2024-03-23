use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize)]
pub struct LtxProjectFormatResult {
  #[serde(rename = "duration")]
  pub duration: u128,
  #[serde(rename = "invalidFiles")]
  pub invalid_files: usize,
  #[serde(rename = "toFormat")]
  pub to_format: Vec<PathBuf>,
  #[serde(rename = "totalFiles")]
  pub total_files: usize,
  #[serde(rename = "validFiles")]
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

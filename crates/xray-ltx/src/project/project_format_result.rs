use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize)]
pub struct LtxProjectFormatResult {
  pub invalid: Vec<PathBuf>,
  pub total: usize,
  pub valid: Vec<PathBuf>,
}

impl LtxProjectFormatResult {
  pub fn new() -> LtxProjectFormatResult {
    LtxProjectFormatResult {
      invalid: Vec::new(),
      total: 0,
      valid: Vec::new(),
    }
  }
}

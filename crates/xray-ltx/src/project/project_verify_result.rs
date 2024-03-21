use crate::LtxSchemeError;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct LtxProjectVerifyResult {
  pub checked_fields: usize,
  pub checked_sections: usize,
  pub duration: u128,
  pub errors: Vec<LtxSchemeError>,
  pub invalid_sections: usize,
  pub skipped_sections: usize,
  pub total_files: usize,
  pub total_sections: usize,
  pub valid_sections: usize,
}

impl LtxProjectVerifyResult {
  pub fn new() -> LtxProjectVerifyResult {
    LtxProjectVerifyResult {
      checked_fields: 0,
      checked_sections: 0,
      duration: 0,
      errors: Vec::new(),
      invalid_sections: 0,
      skipped_sections: 0,
      total_files: 0,
      total_sections: 0,
      valid_sections: 0,
    }
  }
}

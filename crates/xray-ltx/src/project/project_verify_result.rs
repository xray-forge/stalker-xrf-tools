use crate::LtxSchemeError;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct LtxProjectVerifyResult {
  #[serde(rename = "checkedFields")]
  pub checked_fields: usize,
  #[serde(rename = "checkedSections")]
  pub checked_sections: usize,
  #[serde(rename = "duration")]
  pub duration: u128,
  #[serde(rename = "errors")]
  pub errors: Vec<LtxSchemeError>,
  #[serde(rename = "invalidSections")]
  pub invalid_sections: usize,
  #[serde(rename = "skippedSections")]
  pub skipped_sections: usize,
  #[serde(rename = "totalFiles")]
  pub total_files: usize,
  #[serde(rename = "totalSections")]
  pub total_sections: usize,
  #[serde(rename = "validSections")]
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

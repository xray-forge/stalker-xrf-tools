use serde::Serialize;
use xray_error::XRayError;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LtxProjectVerifyResult {
  pub checked_fields: usize,
  pub checked_sections: usize,
  pub duration: u128,
  pub errors: Vec<XRayError>,
  pub invalid_sections: usize,
  pub skipped_sections: usize,
  pub total_files: usize,
  pub total_sections: usize,
  pub valid_sections: usize,
}

impl LtxProjectVerifyResult {
  pub fn new() -> Self {
    Self {
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

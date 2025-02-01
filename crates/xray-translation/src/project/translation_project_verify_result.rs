use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVerifyResult {
  pub duration: u128,
  pub checked_translations_count: u32,
  pub missing_translations_count: u32,
}

impl ProjectVerifyResult {
  pub fn new() -> Self {
    Self {
      duration: 0,
      checked_translations_count: 0,
      missing_translations_count: 0,
    }
  }
}

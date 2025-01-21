use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;
use xray_ltx::{LtxProjectFormatResult, LtxProjectVerifyResult};

#[derive(Default)]
pub struct GamedataLtxFormatVerificationResult {
  pub inner: LtxProjectFormatResult,
}

impl GamedataGenericVerificationResult for GamedataLtxFormatVerificationResult {
  fn is_valid(&self) -> bool {
    self.inner.invalid_files == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} files have invalid format",
      self.inner.invalid_files, self.inner.total_files
    )
  }
}

#[derive(Default)]
pub struct GamedataLtxVerificationResult {
  pub inner: LtxProjectVerifyResult,
}

impl GamedataGenericVerificationResult for GamedataLtxVerificationResult {
  fn is_valid(&self) -> bool {
    self.inner.invalid_sections == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} sections are invalid",
      self.inner.invalid_sections, self.inner.total_sections
    )
  }
}

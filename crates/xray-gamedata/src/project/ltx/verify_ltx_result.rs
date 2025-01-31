use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;
use xray_ltx::{LtxProjectFormatResult, LtxProjectVerifyResult};

#[derive(Default)]
pub struct GamedataLtxVerificationResult {
  pub duration: u128,
  pub format_result: LtxProjectFormatResult,
  pub verification_result: LtxProjectVerifyResult,
}

impl GamedataGenericVerificationResult for GamedataLtxVerificationResult {
  fn is_valid(&self) -> bool {
    self.format_result.invalid_files == 0 && self.verification_result.invalid_sections == 0
  }

  fn get_failure_message(&self) -> String {
    let mut message: String = String::new();

    if self.format_result.invalid_files > 0 {
      message.push_str(&format!(
        "{}/{} files have invalid formatting",
        self.format_result.invalid_files, self.format_result.total_files,
      ))
    }

    if self.verification_result.invalid_sections > 0 {
      if !message.is_empty() {
        message.push_str(", ")
      }

      message.push_str(&format!(
        "{}/{} sections are invalid",
        self.verification_result.invalid_sections, self.verification_result.total_sections
      ))
    }

    message
  }
}

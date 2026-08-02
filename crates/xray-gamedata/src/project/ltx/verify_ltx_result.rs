use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;
use xray_ltx::{LtxProjectFormatResult, LtxProjectVerifyResult};

#[derive(Default)]
pub struct GamedataLtxVerificationResult {
  pub duration: u128,
  pub format_result: LtxProjectFormatResult,
  pub verification_result: LtxProjectVerifyResult,
}

impl GamedataCheckResult for GamedataLtxVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(
      self.format_result.invalid_files == 0 && self.verification_result.invalid_sections == 0,
    )
  }

  fn failure_message(&self) -> String {
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

use crate::{Finding, GamedataCheckResult, GamedataVerificationStatus};
use std::time::Duration;

#[derive(Default)]
pub struct GamedataScriptsVerificationResult {
  pub(crate) duration: Duration,
  pub(crate) invalid_scripts_count: u32,
  pub(crate) checked_scripts_count: u32,
  pub(crate) findings: Vec<Finding>,
}

impl GamedataCheckResult for GamedataScriptsVerificationResult {
  fn duration(&self) -> Option<Duration> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_scripts_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} scripts valid",
      self.checked_scripts_count - self.invalid_scripts_count,
      self.checked_scripts_count
    )
  }

  fn findings(&self) -> &[Finding] {
    &self.findings
  }
}

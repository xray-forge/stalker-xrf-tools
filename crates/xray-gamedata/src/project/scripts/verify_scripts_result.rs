use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataScriptsVerificationResult {
  pub duration: u128,
  pub invalid_scripts_count: u32,
  pub checked_scripts_count: u32,
}

impl GamedataCheckResult for GamedataScriptsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_scripts_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} scripts are invalid",
      self.invalid_scripts_count, self.checked_scripts_count
    )
  }
}

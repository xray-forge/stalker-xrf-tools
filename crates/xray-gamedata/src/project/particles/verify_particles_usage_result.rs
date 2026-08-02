use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataParticlesUsageVerificationResult {
  pub duration: u128,
  pub checked_references_count: u32,
  pub invalid_references_count: u32,
  pub unparsed_custom_data_count: u32,
}

impl GamedataCheckResult for GamedataParticlesUsageVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_references_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} particle references are invalid",
      self.invalid_references_count, self.checked_references_count
    )
  }
}

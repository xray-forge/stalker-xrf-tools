use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataParticlesUsageVerificationResult {
  pub duration: u128,
  pub checked_references_count: u32,
  pub invalid_references_count: u32,
  pub checked_spawn_files_count: u32,
  pub unreadable_spawn_files_count: u32,
  pub unparsed_custom_data_count: u32,
}

impl GamedataCheckResult for GamedataParticlesUsageVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(
      self.invalid_references_count == 0 && self.unreadable_spawn_files_count == 0,
    )
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} particle references are invalid; {}/{} spawn files could not be inspected",
      self.invalid_references_count,
      self.checked_references_count,
      self.unreadable_spawn_files_count,
      self.checked_spawn_files_count
    )
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataParticlesUsageVerificationResult;
  use crate::{GamedataCheckResult, GamedataVerificationStatus};

  #[test]
  fn unreadable_spawn_files_fail_particle_usage_verification() {
    let result: GamedataParticlesUsageVerificationResult =
      GamedataParticlesUsageVerificationResult {
        checked_spawn_files_count: 1,
        unreadable_spawn_files_count: 1,
        ..Default::default()
      };

    assert_eq!(result.status(), GamedataVerificationStatus::Failed);
    assert_eq!(
      result.failure_message(),
      "0/0 particle references are invalid; 1/1 spawn files could not be inspected"
    );
  }
}

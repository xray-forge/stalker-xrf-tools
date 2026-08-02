use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataSoundsVerificationResult {
  pub duration: u128,
  pub findings: Vec<GamedataVerificationFinding>,
  pub invalid_sounds_count: u32,
  pub checked_sounds_count: u32,
}

impl GamedataCheckResult for GamedataSoundsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_sounds_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} sounds valid",
      self.checked_sounds_count - self.invalid_sounds_count,
      self.checked_sounds_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

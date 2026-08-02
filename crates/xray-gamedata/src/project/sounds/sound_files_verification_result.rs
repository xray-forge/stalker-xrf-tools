use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
/// Aggregated outcome of validating individual sound files.
pub(crate) struct GamedataSoundFilesVerificationResult {
  pub(crate) checked_sounds_count: u32,
  pub(crate) findings: Vec<GamedataVerificationFinding>,
  pub(crate) invalid_sounds_count: u32,
}

impl GamedataCheckResult for GamedataSoundFilesVerificationResult {
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

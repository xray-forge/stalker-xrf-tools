use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
/// Aggregated outcome of validating sound references in configs and XML.
pub(crate) struct GamedataSoundReferencesVerificationResult {
  pub(crate) checked_references_count: u32,
  pub(crate) findings: Vec<GamedataVerificationFinding>,
  pub(crate) invalid_references_count: u32,
}

impl GamedataCheckResult for GamedataSoundReferencesVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_references_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} sound references valid",
      self.checked_references_count - self.invalid_references_count,
      self.checked_references_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

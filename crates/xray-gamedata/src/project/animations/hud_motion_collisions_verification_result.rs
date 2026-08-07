use crate::{Finding, GamedataCheckResult, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataHudMotionCollisionsVerificationResult {
  pub checked_huds_count: u32,
  pub collisions_count: u32,
  pub findings: Vec<Finding>,
}

impl GamedataCheckResult for GamedataHudMotionCollisionsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.collisions_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{} motion collisions across {} HUD namespaces",
      self.collisions_count, self.checked_huds_count
    )
  }

  fn findings(&self) -> &[Finding] {
    &self.findings
  }
}

use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataAnimationsVerificationResult {
  pub duration: u128,
  pub invalid_huds_count: u32,
  pub checked_huds_count: u32,
}

impl GamedataCheckResult for GamedataAnimationsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_huds_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} HUD animations are invalid",
      self.invalid_huds_count, self.checked_huds_count
    )
  }
}

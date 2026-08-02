use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataSpawnsVerificationResult {
  pub duration: u128,
  pub total_spawns: u32,
  pub invalid_spawns: u32,
}

impl GamedataCheckResult for GamedataSpawnsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_spawns == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} spawns are invalid",
      self.invalid_spawns, self.total_spawns
    )
  }
}

use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataTexturesVerificationResult {
  pub duration: u128,
  pub invalid_textures_count: u32,
  pub checked_textures_count: u32,
}

impl GamedataCheckResult for GamedataTexturesVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_textures_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} textures are invalid",
      self.invalid_textures_count, self.checked_textures_count
    )
  }
}

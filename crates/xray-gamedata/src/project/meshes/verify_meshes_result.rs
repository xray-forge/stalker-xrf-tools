use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataMeshesVerificationResult {
  pub duration: u128,
  pub invalid_meshes_count: u32,
  pub checked_meshes_count: u32,
}

impl GamedataCheckResult for GamedataMeshesVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_meshes_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} meshes are invalid",
      self.invalid_meshes_count, self.checked_meshes_count
    )
  }
}

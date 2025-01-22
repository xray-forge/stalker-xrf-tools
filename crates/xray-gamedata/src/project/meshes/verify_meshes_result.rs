use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataMeshesVerificationResult {
  pub invalid_meshes: u64,
  pub checked_meshes: u64,
}

impl GamedataGenericVerificationResult for GamedataMeshesVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_meshes == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} meshes are invalid",
      self.invalid_meshes, self.checked_meshes
    )
  }
}

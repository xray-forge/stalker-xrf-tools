use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataMeshesVerificationResult {
  pub duration: u128,
  pub invalid_meshes_count: u32,
  pub checked_meshes_count: u32,
}

impl GamedataGenericVerificationResult for GamedataMeshesVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_meshes_count == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} meshes are invalid",
      self.invalid_meshes_count, self.checked_meshes_count
    )
  }
}

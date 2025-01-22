use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataTexturesVerificationResult {
  pub duration: u128,
  pub invalid_textures_count: u32,
  pub checked_textures_count: u32,
}

impl GamedataGenericVerificationResult for GamedataTexturesVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_textures_count == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} textures are invalid",
      self.invalid_textures_count, self.checked_textures_count
    )
  }
}

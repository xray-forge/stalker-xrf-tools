use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataParticlesVerificationResult {
  pub duration: u128,
  pub checked_particle_files_count: u32,
  pub invalid_particle_files_count: u32,
}

impl GamedataGenericVerificationResult for GamedataParticlesVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_particle_files_count == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} particle files are invalid",
      self.invalid_particle_files_count, self.checked_particle_files_count
    )
  }
}

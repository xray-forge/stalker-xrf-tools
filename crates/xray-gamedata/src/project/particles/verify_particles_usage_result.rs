use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataParticlesUsageVerificationResult {
  pub duration: u128,
  pub checked_references_count: u32,
  pub invalid_references_count: u32,
  pub unparsed_custom_data_count: u32,
}

impl GamedataGenericVerificationResult for GamedataParticlesUsageVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_references_count == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} particle references are invalid",
      self.invalid_references_count, self.checked_references_count
    )
  }
}

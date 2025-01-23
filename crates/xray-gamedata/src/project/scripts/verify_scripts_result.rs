use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataScriptsVerificationResult {
  pub duration: u128,
  pub invalid_scripts_count: u32,
  pub checked_scripts_count: u32,
}

impl GamedataGenericVerificationResult for GamedataScriptsVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_scripts_count == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} scripts are invalid",
      self.invalid_scripts_count, self.checked_scripts_count
    )
  }
}

use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataSpawnsVerificationResult {
  pub duration: u128,
  pub total_spawns: u32,
  pub invalid_spawns: u32,
}

impl GamedataGenericVerificationResult for GamedataSpawnsVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_spawns == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} spawns are invalid",
      self.invalid_spawns, self.total_spawns
    )
  }
}

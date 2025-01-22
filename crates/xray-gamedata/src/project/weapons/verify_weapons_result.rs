use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataWeaponVerificationResult {
  pub checked_weapons_count: u64,
  pub invalid_weapons_count: u64,
}

impl GamedataGenericVerificationResult for GamedataWeaponVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_weapons_count == 0
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} weapons are invalid",
      self.invalid_weapons_count, self.checked_weapons_count
    )
  }
}
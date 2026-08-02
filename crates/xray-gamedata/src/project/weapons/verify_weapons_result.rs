use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataWeaponVerificationResult {
  pub duration: u128,
  pub checked_weapons_count: u32,
  pub invalid_weapons_count: u32,
}

impl GamedataCheckResult for GamedataWeaponVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_weapons_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} weapons valid",
      self.checked_weapons_count - self.invalid_weapons_count,
      self.checked_weapons_count
    )
  }
}

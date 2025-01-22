use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataAnimationsVerificationResult {
  pub duration: u128,
  pub invalid_huds_count: u32,
  pub checked_huds_count: u32,
}

impl GamedataGenericVerificationResult for GamedataAnimationsVerificationResult {
  fn is_valid(&self) -> bool {
    self.invalid_huds_count == 0
  }

  fn get_failure_message(&self) -> String {
    String::from("HUD animations are not valid")
  }
}

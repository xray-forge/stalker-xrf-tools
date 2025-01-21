use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataAnimationsVerificationResult {
  pub is_hud_animation_valid: bool,
}

impl GamedataGenericVerificationResult for GamedataAnimationsVerificationResult {
  fn is_valid(&self) -> bool {
    self.is_hud_animation_valid
  }

  fn get_failure_message(&self) -> String {
    String::from("HUD animations are not valid")
  }
}

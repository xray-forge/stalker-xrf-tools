use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataWeathersVerificationResult {
  pub duration: u128,
}

impl GamedataGenericVerificationResult for GamedataWeathersVerificationResult {
  fn is_valid(&self) -> bool {
    true
  }

  fn get_failure_message(&self) -> String {
    String::from("todo;")
  }
}

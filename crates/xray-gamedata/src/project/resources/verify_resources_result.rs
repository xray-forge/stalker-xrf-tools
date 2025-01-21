use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataResourcesVerificationResult {}

impl GamedataGenericVerificationResult for GamedataResourcesVerificationResult {
  fn is_valid(&self) -> bool {
    true
  }

  fn get_failure_message(&self) -> String {
    String::from("todo;")
  }
}

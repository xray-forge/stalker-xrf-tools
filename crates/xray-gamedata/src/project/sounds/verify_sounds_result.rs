use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataSoundsVerificationResult {}

impl GamedataGenericVerificationResult for GamedataSoundsVerificationResult {
  fn is_valid(&self) -> bool {
    true
  }

  fn get_failure_message(&self) -> String {
    String::from("todo;")
  }
}

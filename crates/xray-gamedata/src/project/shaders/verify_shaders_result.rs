use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataShadersVerificationResult {}

impl GamedataCheckResult for GamedataShadersVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::Passed
  }

  fn failure_message(&self) -> String {
    String::from("todo;")
  }
}

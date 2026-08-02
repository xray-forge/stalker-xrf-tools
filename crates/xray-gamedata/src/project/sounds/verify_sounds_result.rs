use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataSoundsVerificationResult {}

impl GamedataCheckResult for GamedataSoundsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::Passed
  }

  fn failure_message(&self) -> String {
    String::from("todo;")
  }
}

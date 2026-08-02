use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataLevelVerificationResult {}

impl GamedataCheckResult for GamedataLevelVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::Passed
  }

  fn failure_message(&self) -> String {
    String::from("todo;")
  }
}

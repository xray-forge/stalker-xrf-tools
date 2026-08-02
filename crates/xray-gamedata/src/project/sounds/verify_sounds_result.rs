use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataSoundsVerificationResult {}

impl GamedataCheckResult for GamedataSoundsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::Incomplete
  }

  fn failure_message(&self) -> String {
    String::from("Sound validation is not implemented")
  }
}

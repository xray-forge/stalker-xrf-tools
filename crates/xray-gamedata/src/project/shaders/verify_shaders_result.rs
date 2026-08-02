use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataShadersVerificationResult {}

impl GamedataCheckResult for GamedataShadersVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::Incomplete
  }

  fn failure_message(&self) -> String {
    String::from("Shader validation is not implemented")
  }
}

use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataWeathersVerificationResult {
  pub duration: u128,
}

impl GamedataCheckResult for GamedataWeathersVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::Passed
  }

  fn failure_message(&self) -> String {
    String::from("todo;")
  }
}

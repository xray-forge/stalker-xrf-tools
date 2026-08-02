use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataWeathersVerificationResult {
  pub duration: u128,
}

impl GamedataCheckResult for GamedataWeathersVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::Incomplete
  }

  fn failure_message(&self) -> String {
    String::from("Weather validation parses files but does not validate their semantics")
  }
}

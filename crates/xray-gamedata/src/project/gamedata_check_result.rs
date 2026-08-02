use crate::GamedataVerificationStatus;

pub trait GamedataCheckResult {
  fn status(&self) -> GamedataVerificationStatus;

  fn failure_message(&self) -> String;
}

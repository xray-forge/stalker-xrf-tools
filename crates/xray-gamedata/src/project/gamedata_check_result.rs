use crate::{GamedataVerificationFinding, GamedataVerificationStatus};

pub trait GamedataCheckResult {
  fn status(&self) -> GamedataVerificationStatus;

  fn failure_message(&self) -> String;

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &[]
  }
}

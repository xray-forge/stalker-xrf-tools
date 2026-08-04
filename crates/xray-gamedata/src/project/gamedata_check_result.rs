use crate::{GamedataVerificationFinding, GamedataVerificationStatus};

pub trait GamedataCheckResult {
  fn duration(&self) -> Option<u128> {
    None
  }

  fn status(&self) -> GamedataVerificationStatus;

  fn failure_message(&self) -> String;

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &[]
  }
}

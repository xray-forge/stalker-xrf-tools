use crate::{Finding, GamedataVerificationStatus};
use std::time::Duration;

pub trait GamedataCheckResult {
  fn duration(&self) -> Option<Duration> {
    None
  }

  fn status(&self) -> GamedataVerificationStatus;

  fn failure_message(&self) -> String;

  fn findings(&self) -> &[Finding] {
    &[]
  }
}

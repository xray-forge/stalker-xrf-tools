use std::time::Duration;

use crate::{Finding, GamedataVerificationStatus};

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

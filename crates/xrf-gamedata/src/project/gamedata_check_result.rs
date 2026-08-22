use std::time::Duration;

use crate::{Finding, GamedataVerificationStatus};

pub trait GamedataCheckResult {
  fn get_duration(&self) -> Option<Duration> {
    None
  }

  fn get_status(&self) -> GamedataVerificationStatus;

  fn get_failure_message(&self) -> String;

  fn get_findings(&self) -> &[Finding] {
    &[]
  }
}

use crate::{CheckId, Finding, Status};
use serde::Serialize;
use std::time::Duration;

/// Immutable report data for one command check.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckReport {
  duration: Option<Duration>,
  findings: Vec<Finding>,
  id: CheckId,
  status: Status,
}

impl CheckReport {
  pub fn new(
    id: CheckId,
    status: Status,
    duration: Option<Duration>,
    findings: Vec<Finding>,
  ) -> Self {
    Self {
      duration,
      findings,
      id,
      status,
    }
  }

  pub const fn duration(&self) -> Option<Duration> {
    self.duration
  }

  pub fn findings(&self) -> &[Finding] {
    &self.findings
  }

  pub fn id(&self) -> &CheckId {
    &self.id
  }

  pub const fn status(&self) -> Status {
    self.status
  }

  pub(crate) fn finalize(&mut self) {
    self.findings.sort_by(Finding::cmp);
  }
}

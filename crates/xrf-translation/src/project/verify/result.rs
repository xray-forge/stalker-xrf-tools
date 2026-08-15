use std::path::Path;
use std::time::Duration;

use serde::Serialize;
use xrf_report::{CheckId, CheckReport, Finding, Report, RuleId, Status};

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVerifyResult {
  pub duration: u128,
  pub checked_translations_count: u32,
  pub missing_translations_count: u32,
  #[serde(skip_serializing)]
  findings: Vec<Finding>,
}

impl ProjectVerifyResult {
  pub fn new() -> Self {
    Self {
      duration: 0,
      checked_translations_count: 0,
      missing_translations_count: 0,
      findings: Vec::new(),
    }
  }

  pub fn findings(&self) -> &[Finding] {
    &self.findings
  }

  pub fn status(&self) -> Status {
    Status::from_is_valid(self.missing_translations_count == 0)
  }

  pub fn to_report(&self) -> Report {
    let duration_millis: u64 = self.duration.min(u128::from(u64::MAX)) as u64;

    Report::new(vec![CheckReport::new(
      CheckId::new("translations").expect("Expected a non-empty translation check ID"),
      self.status(),
      Some(Duration::from_millis(duration_millis)),
      self.findings.clone(),
    )])
  }

  pub(crate) fn merge(&mut self, other: Self) {
    self.checked_translations_count += other.checked_translations_count;
    self.missing_translations_count += other.missing_translations_count;
    self.findings.extend(other.findings);
  }

  pub(crate) fn record_missing_translation(&mut self, path: &Path, key: &str, language: &str) {
    self.missing_translations_count += 1;
    self.findings.push(Finding::new(
      RuleId::new("translations.missing").expect("Expected a non-empty translation rule ID"),
      Some(path.to_string_lossy().replace('\\', "/")),
      format!("Missing translation for key '{key}' in language '{language}'"),
    ));
  }
}

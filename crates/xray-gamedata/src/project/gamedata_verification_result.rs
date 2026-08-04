use crate::{
  GamedataCheckResult, GamedataVerificationRule, GamedataVerificationStatus,
  GamedataVerificationType,
};
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::time::Duration;
use xray_error::XRayResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GamedataVerificationFinding {
  asset_path: Option<PathBuf>,
  message: String,
  rule: GamedataVerificationRule,
}

impl GamedataVerificationFinding {
  pub fn for_asset<P, M>(rule: GamedataVerificationRule, asset_path: P, message: M) -> Self
  where
    P: AsRef<Path>,
    M: Into<String>,
  {
    Self {
      asset_path: Some(asset_path.as_ref().to_path_buf()),
      message: message.into(),
      rule,
    }
  }

  pub fn without_asset<M>(rule: GamedataVerificationRule, message: M) -> Self
  where
    M: Into<String>,
  {
    Self {
      asset_path: None,
      message: message.into(),
      rule,
    }
  }

  pub fn asset_path(&self) -> Option<&Path> {
    self.asset_path.as_deref()
  }

  pub fn message(&self) -> &str {
    &self.message
  }

  pub const fn rule(&self) -> GamedataVerificationRule {
    self.rule
  }

  /// Orders findings by asset path, then message.
  pub fn cmp_by_asset_path_and_message(left: &Self, right: &Self) -> Ordering {
    left
      .asset_path
      .cmp(&right.asset_path)
      .then_with(|| left.message.cmp(&right.message))
  }

  /// Orders findings by asset path, rule, then message.
  pub fn cmp_by_asset_path_rule_and_message(left: &Self, right: &Self) -> Ordering {
    left
      .asset_path
      .cmp(&right.asset_path)
      .then_with(|| left.rule.cmp(&right.rule))
      .then_with(|| left.message.cmp(&right.message))
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GamedataVerificationCheckReport {
  duration: Option<Duration>,
  findings: Vec<GamedataVerificationFinding>,
  status: GamedataVerificationStatus,
  summary: String,
  verification_type: GamedataVerificationType,
}

#[derive(Default)]
pub struct GamedataVerificationReport {
  checks: Vec<GamedataVerificationCheckReport>,
  duration: Duration,
}

pub type GamedataVerificationResult = GamedataVerificationReport;

impl GamedataVerificationReport {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_duration(duration: Duration) -> Self {
    Self {
      duration,
      ..Self::default()
    }
  }

  pub fn checks(&self) -> &[GamedataVerificationCheckReport] {
    &self.checks
  }

  pub const fn duration(&self) -> Duration {
    self.duration
  }

  pub(crate) fn set_duration(&mut self, duration: Duration) {
    self.duration = duration;
  }

  pub(crate) fn add_report(&mut self, report: GamedataVerificationCheckReport) {
    self.checks.push(report);
  }

  pub fn add_check<T>(&mut self, verification_type: GamedataVerificationType, result: XRayResult<T>)
  where
    T: GamedataCheckResult,
  {
    self
      .checks
      .push(GamedataVerificationCheckReport::from_check_result(
        verification_type,
        result,
      ));
  }

  pub fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::aggregate(self.checks.iter().map(|it| it.status))
  }

  pub fn is_valid(&self) -> bool {
    self.status() == GamedataVerificationStatus::Passed
  }

  pub fn get_failure_messages(&self) -> Vec<String> {
    self
      .get_failure_reports()
      .map(|it| it.summary.clone())
      .collect()
  }

  pub fn get_failure_reports(&self) -> impl Iterator<Item = &GamedataVerificationCheckReport> {
    self.checks.iter().filter(|it| {
      !matches!(
        it.status,
        GamedataVerificationStatus::Passed | GamedataVerificationStatus::Skipped
      )
    })
  }
}

impl GamedataVerificationCheckReport {
  pub fn duration(&self) -> Option<Duration> {
    self.duration
  }

  pub fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }

  pub const fn status(&self) -> GamedataVerificationStatus {
    self.status
  }

  pub fn summary(&self) -> &str {
    &self.summary
  }

  pub const fn verification_type(&self) -> GamedataVerificationType {
    self.verification_type
  }

  pub(crate) fn from_check_result<T>(
    verification_type: GamedataVerificationType,
    result: XRayResult<T>,
  ) -> Self
  where
    T: GamedataCheckResult,
  {
    match result {
      Ok(result) => Self {
        duration: result.duration(),
        findings: result.findings().to_vec(),
        status: result.status(),
        summary: result.failure_message(),
        verification_type,
      },
      Err(error) => Self {
        duration: None,
        findings: vec![GamedataVerificationFinding::without_asset(
          GamedataVerificationRule::CheckExecution,
          error.to_string(),
        )],
        status: GamedataVerificationStatus::Error,
        summary: format!("Check failed ({verification_type}): {error}"),
        verification_type,
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{GamedataVerificationFinding, GamedataVerificationReport};
  use crate::{
    GamedataCheckResult, GamedataVerificationRule, GamedataVerificationStatus,
    GamedataVerificationType,
  };
  use xray_error::XRayError;

  struct TestCheckResult {
    findings: Vec<GamedataVerificationFinding>,
    status: GamedataVerificationStatus,
    summary: String,
  }

  impl GamedataCheckResult for TestCheckResult {
    fn status(&self) -> GamedataVerificationStatus {
      self.status
    }

    fn failure_message(&self) -> String {
      self.summary.clone()
    }

    fn findings(&self) -> &[GamedataVerificationFinding] {
      &self.findings
    }
  }

  #[test]
  fn sorts_findings_by_asset_path_then_message() {
    let mut findings: Vec<GamedataVerificationFinding> = vec![
      GamedataVerificationFinding::for_asset(
        GamedataVerificationRule::ScriptsSyntax,
        "scripts/a.script",
        "Second",
      ),
      GamedataVerificationFinding::for_asset(
        GamedataVerificationRule::ScriptsSyntax,
        "scripts/z.script",
        "First",
      ),
      GamedataVerificationFinding::for_asset(
        GamedataVerificationRule::ScriptsSyntax,
        "scripts/a.script",
        "First",
      ),
    ];

    findings.sort_by(GamedataVerificationFinding::cmp_by_asset_path_and_message);

    assert_eq!(
      findings
        .iter()
        .map(GamedataVerificationFinding::message)
        .collect::<Vec<_>>(),
      vec!["First", "Second", "First"]
    );
  }

  #[test]
  fn sorts_equal_paths_by_rule_before_message() {
    let scripts_finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      GamedataVerificationRule::ScriptsSyntax,
      "scripts/a.script",
      "First",
    );
    let textures_finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      GamedataVerificationRule::TexturesRead,
      "scripts/a.script",
      "Second",
    );
    let mut findings: Vec<GamedataVerificationFinding> =
      vec![textures_finding.clone(), scripts_finding.clone()];

    findings.sort_by(GamedataVerificationFinding::cmp_by_asset_path_rule_and_message);

    assert_eq!(findings, vec![scripts_finding, textures_finding]);
  }

  #[test]
  fn empty_verification_result_is_skipped_and_not_valid() {
    let result = GamedataVerificationReport::default();

    assert_eq!(result.status(), GamedataVerificationStatus::Skipped);
    assert!(!result.is_valid());
  }

  #[test]
  fn collects_check_summaries_and_findings() {
    let mut result = GamedataVerificationReport::default();

    result.add_check(
      GamedataVerificationType::Scripts,
      Ok(TestCheckResult {
        findings: vec![GamedataVerificationFinding::for_asset(
          GamedataVerificationRule::ScriptsSyntax,
          "scripts/invalid.script",
          "Expected expression after '='",
        )],
        status: GamedataVerificationStatus::Failed,
        summary: String::from("1/1 scripts are invalid"),
      }),
    );

    assert_eq!(result.status(), GamedataVerificationStatus::Failed);
    assert_eq!(
      result.get_failure_messages(),
      vec![String::from("1/1 scripts are invalid")]
    );
    assert_eq!(
      result.checks()[0].findings(),
      vec![GamedataVerificationFinding::for_asset(
        GamedataVerificationRule::ScriptsSyntax,
        "scripts/invalid.script",
        "Expected expression after '='",
      )]
    );
  }

  #[test]
  fn records_checker_errors_as_findings() {
    let mut result = GamedataVerificationReport::default();

    result.add_check::<TestCheckResult>(
      GamedataVerificationType::Animations,
      Err(XRayError::new_unexpected_error("boom")),
    );

    assert_eq!(result.status(), GamedataVerificationStatus::Error);
    assert_eq!(
      result.get_failure_messages(),
      vec![String::from(
        "Check failed (animations): Unexpected error: boom"
      )]
    );
    assert_eq!(
      result.checks()[0].findings(),
      vec![GamedataVerificationFinding::without_asset(
        GamedataVerificationRule::CheckExecution,
        "Unexpected error: boom",
      )]
    );
  }
}

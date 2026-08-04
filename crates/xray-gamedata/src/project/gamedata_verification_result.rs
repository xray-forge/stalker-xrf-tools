use crate::{GamedataCheckResult, GamedataVerificationStatus, GamedataVerificationType};
use std::path::{Path, PathBuf};
use xray_error::XRayResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GamedataVerificationFinding {
  pub asset_path: Option<PathBuf>,
  pub message: String,
  pub rule_id: Option<String>,
}

impl GamedataVerificationFinding {
  pub fn for_asset<P, M>(asset_path: P, message: M) -> Self
  where
    P: AsRef<Path>,
    M: Into<String>,
  {
    Self {
      asset_path: Some(asset_path.as_ref().to_path_buf()),
      message: message.into(),
      rule_id: None,
    }
  }

  pub fn for_asset_in_rule<R, P, M>(rule_id: R, asset_path: P, message: M) -> Self
  where
    R: Into<String>,
    P: AsRef<Path>,
    M: Into<String>,
  {
    Self {
      asset_path: Some(asset_path.as_ref().to_path_buf()),
      message: message.into(),
      rule_id: Some(rule_id.into()),
    }
  }

  pub fn without_asset<M>(message: M) -> Self
  where
    M: Into<String>,
  {
    Self {
      asset_path: None,
      message: message.into(),
      rule_id: None,
    }
  }

  pub fn without_asset_in_rule<R, M>(rule_id: R, message: M) -> Self
  where
    R: Into<String>,
    M: Into<String>,
  {
    Self {
      asset_path: None,
      message: message.into(),
      rule_id: Some(rule_id.into()),
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GamedataVerificationCheckReport {
  pub findings: Vec<GamedataVerificationFinding>,
  pub status: GamedataVerificationStatus,
  pub summary: String,
  pub verification_type: GamedataVerificationType,
}

#[derive(Default)]
pub struct GamedataVerificationReport {
  pub checks: Vec<GamedataVerificationCheckReport>,
  pub duration: u128,
}

pub type GamedataVerificationResult = GamedataVerificationReport;

impl GamedataVerificationReport {
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
  pub fn from_check_result<T>(
    verification_type: GamedataVerificationType,
    result: XRayResult<T>,
  ) -> Self
  where
    T: GamedataCheckResult,
  {
    match result {
      Ok(result) => Self {
        findings: result.findings().to_vec(),
        status: result.status(),
        summary: result.failure_message(),
        verification_type,
      },
      Err(error) => Self {
        findings: vec![GamedataVerificationFinding::without_asset(
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
  use crate::{GamedataCheckResult, GamedataVerificationStatus, GamedataVerificationType};
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
      result.checks[0].findings,
      vec![GamedataVerificationFinding::for_asset(
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
      result.checks[0].findings,
      vec![GamedataVerificationFinding::without_asset(
        "Unexpected error: boom",
      )]
    );
  }
}

use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};
use std::time::Duration;
use xray_ltx::{LtxProjectFormatResult, LtxProjectVerifyResult};

#[derive(Default)]
pub struct GamedataLtxVerificationResult {
  pub(crate) duration: Duration,
  pub(crate) findings: Vec<GamedataVerificationFinding>,
  pub(crate) format_result: LtxProjectFormatResult,
  pub(crate) verification_result: LtxProjectVerifyResult,
}

impl GamedataCheckResult for GamedataLtxVerificationResult {
  fn duration(&self) -> Option<Duration> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(
      self.format_result.invalid_files == 0 && self.verification_result.invalid_sections == 0,
    )
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} LTX files formatted; {}/{} sections valid",
      self.format_result.valid_files,
      self.format_result.total_files,
      self.verification_result.valid_sections,
      self.verification_result.total_sections
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataLtxVerificationResult;
  use crate::{
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationRule,
    GamedataVerificationStatus, GamedataVerificationType,
  };
  use xray_ltx::LtxProjectVerifyResult;

  #[test]
  fn exposes_ltx_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      GamedataVerificationRule::LtxFormatting,
      "configs/system.ltx",
      "LTX file needs formatting",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Ltx,
      Ok(GamedataLtxVerificationResult {
        findings: vec![finding.clone()],
        verification_result: LtxProjectVerifyResult {
          invalid_sections: 1,
          ..Default::default()
        },
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks()[0].findings(), [finding.into_report()]);
  }
}

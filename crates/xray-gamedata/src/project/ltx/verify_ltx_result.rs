use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};
use xray_ltx::{LtxProjectFormatResult, LtxProjectVerifyResult};

#[derive(Default)]
pub struct GamedataLtxVerificationResult {
  pub duration: u128,
  pub findings: Vec<GamedataVerificationFinding>,
  pub format_result: LtxProjectFormatResult,
  pub verification_result: LtxProjectVerifyResult,
}

impl GamedataCheckResult for GamedataLtxVerificationResult {
  fn duration(&self) -> Option<u128> {
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
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationStatus,
    GamedataVerificationType,
  };
  use xray_ltx::LtxProjectVerifyResult;

  #[test]
  fn exposes_ltx_findings_in_reports() {
    let finding: GamedataVerificationFinding =
      GamedataVerificationFinding::for_asset_in_rule(
        "ltx.unclassified",
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
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

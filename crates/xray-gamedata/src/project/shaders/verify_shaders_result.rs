use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataShadersVerificationResult {
  pub duration: u128,
  checked_scripts_count: u32,
  checked_sources_count: u32,
  findings: Vec<GamedataVerificationFinding>,
}

impl GamedataShadersVerificationResult {
  pub(crate) fn add_finding(&mut self, finding: GamedataVerificationFinding) {
    self.findings.push(finding);
  }

  pub(crate) fn increment_checked_scripts_count(&mut self) {
    self.checked_scripts_count += 1;
  }

  pub(crate) fn increment_checked_sources_count(&mut self) {
    self.checked_sources_count += 1;
  }

  pub(crate) fn sort_findings(&mut self) {
    self.findings.sort_by(|left, right| {
      left
        .asset_path
        .cmp(&right.asset_path)
        .then_with(|| left.rule_id.cmp(&right.rule_id))
        .then_with(|| left.message.cmp(&right.message))
    });
  }
}

impl GamedataCheckResult for GamedataShadersVerificationResult {
  fn duration(&self) -> Option<u128> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.findings.is_empty())
  }

  fn failure_message(&self) -> String {
    format!(
      "{} shader scripts and {} shader sources checked, {} problems",
      self.checked_scripts_count,
      self.checked_sources_count,
      self.findings.len()
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataShadersVerificationResult;
  use crate::{
    GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationReport,
    GamedataVerificationStatus, GamedataVerificationType,
  };

  #[test]
  fn exposes_renderer_shader_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset_in_rule(
      "shaders.include-missing",
      "shaders/r3/main.ps",
      "Shader source includes missing file 'common.h'",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Shaders,
      Ok(GamedataShadersVerificationResult {
        checked_sources_count: 1,
        findings: vec![finding.clone()],
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }

  #[test]
  fn summarizes_checked_renderer_shader_files() {
    let result: GamedataShadersVerificationResult = GamedataShadersVerificationResult {
      checked_scripts_count: 2,
      checked_sources_count: 3,
      ..Default::default()
    };

    assert_eq!(
      result.failure_message(),
      "2 shader scripts and 3 shader sources checked, 0 problems"
    );
  }
}

use std::time::Duration;

use crate::{Finding, GamedataCheckResult, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataTexturesVerificationResult {
  pub(crate) duration: Duration,
  pub(crate) findings: Vec<Finding>,
  pub(crate) invalid_textures_count: u32,
  pub(crate) checked_textures_count: u32,
  /// Texture descriptors that declare a bump the engine can resolve to a file.
  pub(crate) checked_bumps_count: u32,
  pub(crate) unresolved_bumps_count: u32,
}

impl GamedataCheckResult for GamedataTexturesVerificationResult {
  fn get_duration(&self) -> Option<Duration> {
    Some(self.duration)
  }

  fn get_status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_textures_count == 0 && self.unresolved_bumps_count == 0)
  }

  fn get_failure_message(&self) -> String {
    format!(
      "{}/{} textures valid, {}/{} declared bumps resolved",
      self.checked_textures_count - self.invalid_textures_count,
      self.checked_textures_count,
      self.checked_bumps_count - self.unresolved_bumps_count,
      self.checked_bumps_count
    )
  }

  fn get_findings(&self) -> &[Finding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataTexturesVerificationResult;
  use crate::GamedataFindingFactory;
  use crate::{
    Finding, GamedataVerificationReport, GamedataVerificationRule, GamedataVerificationStatus, GamedataVerificationType,
  };

  #[test]
  fn exposes_texture_findings_in_reports() {
    let finding: Finding = GamedataFindingFactory::for_asset(
      GamedataVerificationRule::TexturesValidation,
      "textures/test.dds",
      "Texture uses an unsupported format",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Textures,
      Ok(GamedataTexturesVerificationResult {
        checked_textures_count: 1,
        findings: vec![finding.clone()],
        invalid_textures_count: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.get_status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.get_checks()[0].get_findings(), [finding]);
  }
}

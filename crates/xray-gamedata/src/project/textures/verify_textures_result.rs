use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataTexturesVerificationResult {
  pub duration: u128,
  pub findings: Vec<GamedataVerificationFinding>,
  pub invalid_textures_count: u32,
  pub checked_textures_count: u32,
}

impl GamedataCheckResult for GamedataTexturesVerificationResult {
  fn duration(&self) -> Option<u128> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_textures_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} textures valid",
      self.checked_textures_count - self.invalid_textures_count,
      self.checked_textures_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataTexturesVerificationResult;
  use crate::{
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationRule,
    GamedataVerificationStatus, GamedataVerificationType,
  };

  #[test]
  fn exposes_texture_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
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

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataMeshesVerificationResult {
  pub duration: u128,
  pub findings: Vec<GamedataVerificationFinding>,
  pub invalid_meshes_count: u32,
  pub checked_meshes_count: u32,
}

impl GamedataCheckResult for GamedataMeshesVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_meshes_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} meshes valid",
      self.checked_meshes_count - self.invalid_meshes_count,
      self.checked_meshes_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataMeshesVerificationResult;
  use crate::{
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationStatus,
    GamedataVerificationType,
  };

  #[test]
  fn exposes_mesh_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      "meshes/test.ogf",
      "Mesh references missing texture 'textures/test'",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Meshes,
      Ok(GamedataMeshesVerificationResult {
        checked_meshes_count: 1,
        findings: vec![finding.clone()],
        invalid_meshes_count: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

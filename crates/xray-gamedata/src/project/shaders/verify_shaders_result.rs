use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataShadersVerificationResult {
  pub blender_count: usize,
  pub checked_shader_libraries_count: u32,
  pub duration: u128,
  pub findings: Vec<GamedataVerificationFinding>,
  pub invalid_shader_libraries_count: u32,
}

impl GamedataCheckResult for GamedataShadersVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_shader_libraries_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} shader libraries valid, {} blender definitions",
      self.checked_shader_libraries_count - self.invalid_shader_libraries_count,
      self.checked_shader_libraries_count,
      self.blender_count
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
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationStatus,
    GamedataVerificationType,
  };

  #[test]
  fn exposes_shader_library_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      "shaders.xr",
      "Failed to read shader library: missing blenders chunk",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Shaders,
      Ok(GamedataShadersVerificationResult {
        checked_shader_libraries_count: 1,
        findings: vec![finding.clone()],
        invalid_shader_libraries_count: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

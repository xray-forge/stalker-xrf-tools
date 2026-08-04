use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataParticlesUsageVerificationResult {
  pub duration: u128,
  pub checked_references_count: u32,
  pub findings: Vec<GamedataVerificationFinding>,
  pub invalid_references_count: u32,
  pub checked_spawn_files_count: u32,
  pub unreadable_spawn_files_count: u32,
  pub unparsed_custom_data_count: u32,
}

impl GamedataCheckResult for GamedataParticlesUsageVerificationResult {
  fn duration(&self) -> Option<u128> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(
      self.invalid_references_count == 0 && self.unreadable_spawn_files_count == 0,
    )
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} particle references valid; {}/{} spawn files readable",
      self.checked_references_count - self.invalid_references_count,
      self.checked_references_count,
      self.checked_spawn_files_count - self.unreadable_spawn_files_count,
      self.checked_spawn_files_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataParticlesUsageVerificationResult;
  use crate::{
    GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationReport,
    GamedataVerificationStatus, GamedataVerificationType,
  };

  #[test]
  fn unreadable_spawn_files_fail_particle_usage_verification() {
    let result: GamedataParticlesUsageVerificationResult =
      GamedataParticlesUsageVerificationResult {
        checked_spawn_files_count: 1,
        unreadable_spawn_files_count: 1,
        ..Default::default()
      };

    assert_eq!(result.status(), GamedataVerificationStatus::Failed);
    assert_eq!(
      result.failure_message(),
      "0/0 particle references valid; 0/1 spawn files readable"
    );
  }

  #[test]
  fn exposes_particle_usage_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      "configs/scripts/test.ltx",
      "Unknown particle reference: [sr_particle] name = missing_particle",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::ParticlesUsage,
      Ok(GamedataParticlesUsageVerificationResult {
        checked_references_count: 1,
        findings: vec![finding.clone()],
        invalid_references_count: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

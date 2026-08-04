use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataSpawnsVerificationResult {
  pub duration: u128,
  pub findings: Vec<GamedataVerificationFinding>,
  pub total_spawns: u32,
  pub invalid_spawns: u32,
}

impl GamedataCheckResult for GamedataSpawnsVerificationResult {
  fn duration(&self) -> Option<u128> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_spawns == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} spawns valid",
      self.total_spawns - self.invalid_spawns,
      self.total_spawns
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataSpawnsVerificationResult;
  use crate::{
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationStatus,
    GamedataVerificationType,
  };

  #[test]
  fn exposes_spawn_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      "spawns/test.spawn",
      "Failed to read spawn file: invalid header",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Spawns,
      Ok(GamedataSpawnsVerificationResult {
        findings: vec![finding.clone()],
        invalid_spawns: 1,
        total_spawns: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

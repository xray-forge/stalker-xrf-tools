use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataAnimationsVerificationResult {
  pub duration: u128,
  pub findings: Vec<GamedataVerificationFinding>,
  pub invalid_huds_count: u32,
  pub checked_huds_count: u32,
}

impl GamedataCheckResult for GamedataAnimationsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_huds_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} HUD animations valid",
      self.checked_huds_count - self.invalid_huds_count,
      self.checked_huds_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataAnimationsVerificationResult;
  use crate::{
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationStatus,
    GamedataVerificationType,
  };

  #[test]
  fn exposes_animation_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      "configs/system.ltx",
      "Player HUD section [actor_hud] has invalid animations",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Animations,
      Ok(GamedataAnimationsVerificationResult {
        checked_huds_count: 1,
        findings: vec![finding.clone()],
        invalid_huds_count: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

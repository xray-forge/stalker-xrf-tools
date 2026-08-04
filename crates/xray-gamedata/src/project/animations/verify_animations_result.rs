use crate::project::animations::player_hud_animations_verification_result::GamedataPlayerHudAnimationsVerificationResult;
use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

pub struct GamedataAnimationsVerificationResult {
  pub duration: u128,
  pub(crate) findings: Vec<GamedataVerificationFinding>,
  pub(crate) player_hud_animations: GamedataPlayerHudAnimationsVerificationResult,
}

impl GamedataCheckResult for GamedataAnimationsVerificationResult {
  fn duration(&self) -> Option<u128> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    self.player_hud_animations.status()
  }

  fn failure_message(&self) -> String {
    self.player_hud_animations.failure_message()
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataAnimationsVerificationResult;
  use crate::project::animations::player_hud_animations_verification_result::GamedataPlayerHudAnimationsVerificationResult;
  use crate::{
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationStatus,
    GamedataVerificationType,
  };

  #[test]
  fn exposes_animation_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset_in_rule(
      "animations.player-hud",
      "configs/system.ltx",
      "Player HUD section [actor_hud] has invalid animations",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Animations,
      Ok(GamedataAnimationsVerificationResult {
        duration: 0,
        findings: vec![finding.clone()],
        player_hud_animations: GamedataPlayerHudAnimationsVerificationResult {
          checked_huds_count: 1,
          findings: vec![finding.clone()],
          invalid_huds_count: 1,
        },
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

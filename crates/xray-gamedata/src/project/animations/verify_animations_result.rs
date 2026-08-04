use crate::project::animations::player_hud_animations_verification_result::GamedataPlayerHudAnimationsVerificationResult;
use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};
use std::time::Duration;

pub struct GamedataAnimationsVerificationResult {
  pub(crate) duration: Duration,
  pub(crate) findings: Vec<GamedataVerificationFinding>,
  pub(crate) player_hud_animations: GamedataPlayerHudAnimationsVerificationResult,
}

impl GamedataCheckResult for GamedataAnimationsVerificationResult {
  fn duration(&self) -> Option<Duration> {
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
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationRule,
    GamedataVerificationStatus, GamedataVerificationType,
  };
  use std::time::Duration;

  #[test]
  fn exposes_animation_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      GamedataVerificationRule::AnimationsPlayerHud,
      "configs/system.ltx",
      "Player HUD section [actor_hud] has invalid animations",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Animations,
      Ok(GamedataAnimationsVerificationResult {
        duration: Duration::ZERO,
        findings: vec![finding.clone()],
        player_hud_animations: GamedataPlayerHudAnimationsVerificationResult {
          checked_huds_count: 1,
          findings: vec![finding.clone()],
          invalid_huds_count: 1,
        },
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks()[0].findings(), [finding]);
  }
}

use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};
use std::time::Duration;

#[derive(Default)]
pub struct GamedataWeaponVerificationResult {
  pub(crate) duration: Duration,
  pub(crate) checked_weapons_count: u32,
  pub(crate) findings: Vec<GamedataVerificationFinding>,
  pub(crate) invalid_weapons_count: u32,
}

impl GamedataCheckResult for GamedataWeaponVerificationResult {
  fn duration(&self) -> Option<Duration> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_weapons_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} weapons valid",
      self.checked_weapons_count - self.invalid_weapons_count,
      self.checked_weapons_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataWeaponVerificationResult;
  use crate::{
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationRule,
    GamedataVerificationStatus, GamedataVerificationType,
  };

  #[test]
  fn exposes_weapon_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      GamedataVerificationRule::WeaponsValidation,
      "configs/system.ltx",
      "Weapon section [wpn_test] is invalid",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Weapons,
      Ok(GamedataWeaponVerificationResult {
        checked_weapons_count: 1,
        findings: vec![finding.clone()],
        invalid_weapons_count: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks()[0].findings(), [finding]);
  }
}

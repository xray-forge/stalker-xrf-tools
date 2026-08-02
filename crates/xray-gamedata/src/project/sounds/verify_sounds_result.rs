use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataSoundsVerificationResult {
  pub checked_sound_references_count: u32,
  pub duration: u128,
  pub findings: Vec<GamedataVerificationFinding>,
  pub invalid_sound_references_count: u32,
  pub invalid_sounds_count: u32,
  pub checked_sounds_count: u32,
}

impl GamedataCheckResult for GamedataSoundsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(
      self.invalid_sounds_count == 0 && self.invalid_sound_references_count == 0,
    )
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} sounds valid; {}/{} sound references valid",
      self.checked_sounds_count - self.invalid_sounds_count,
      self.checked_sounds_count,
      self.checked_sound_references_count - self.invalid_sound_references_count,
      self.checked_sound_references_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataSoundsVerificationResult;
  use crate::{
    GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationReport,
    GamedataVerificationStatus, GamedataVerificationType,
  };

  #[test]
  fn exposes_sound_reference_findings_in_sound_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      "configs/ui/game_tutorials.xml",
      "Unknown sound reference: <sound> = video\\missing",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Sounds,
      Ok(GamedataSoundsVerificationResult {
        checked_sound_references_count: 1,
        findings: vec![finding.clone()],
        invalid_sound_references_count: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
    assert_eq!(
      report.checks[0].summary,
      "0/0 sounds valid; 0/1 sound references valid"
    );
  }

  #[test]
  fn fails_when_a_sound_reference_is_invalid() {
    let result: GamedataSoundsVerificationResult = GamedataSoundsVerificationResult {
      checked_sound_references_count: 1,
      invalid_sound_references_count: 1,
      ..Default::default()
    };

    assert_eq!(result.status(), GamedataVerificationStatus::Failed);
  }
}

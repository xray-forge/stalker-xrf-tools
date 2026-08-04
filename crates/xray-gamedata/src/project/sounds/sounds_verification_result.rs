use crate::project::sounds::sound_files_verification_result::GamedataSoundFilesVerificationResult;
use crate::project::sounds::sound_references_verification_result::GamedataSoundReferencesVerificationResult;
use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

pub struct GamedataSoundsVerificationResult {
  pub duration: u128,
  findings: Vec<GamedataVerificationFinding>,
  sound_files: GamedataSoundFilesVerificationResult,
  sound_references: GamedataSoundReferencesVerificationResult,
}

impl GamedataSoundsVerificationResult {
  pub(crate) fn new(
    duration: u128,
    sound_files: GamedataSoundFilesVerificationResult,
    sound_references: GamedataSoundReferencesVerificationResult,
  ) -> Self {
    let mut findings: Vec<GamedataVerificationFinding> = sound_files
      .findings()
      .iter()
      .chain(sound_references.findings())
      .cloned()
      .collect();

    findings.sort_by(|left, right| {
      left
        .asset_path
        .cmp(&right.asset_path)
        .then_with(|| left.rule.cmp(&right.rule))
        .then_with(|| left.message.cmp(&right.message))
    });

    Self {
      duration,
      findings,
      sound_files,
      sound_references,
    }
  }
}

impl GamedataCheckResult for GamedataSoundsVerificationResult {
  fn duration(&self) -> Option<u128> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::aggregate([
      self.sound_files.status(),
      self.sound_references.status(),
    ])
  }

  fn failure_message(&self) -> String {
    format!(
      "{}; {}",
      self.sound_files.failure_message(),
      self.sound_references.failure_message(),
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataSoundsVerificationResult;
  use crate::project::sounds::sound_files_verification_result::GamedataSoundFilesVerificationResult;
  use crate::project::sounds::sound_references_verification_result::GamedataSoundReferencesVerificationResult;
  use crate::{
    GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationReport,
    GamedataVerificationRule, GamedataVerificationStatus, GamedataVerificationType,
  };

  #[test]
  fn exposes_sound_reference_findings_in_sound_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      GamedataVerificationRule::SoundsReferences,
      "configs/ui/game_tutorials.xml",
      "Unknown sound reference: <sound> = video\\missing",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Sounds,
      Ok(GamedataSoundsVerificationResult::new(
        0,
        GamedataSoundFilesVerificationResult::default(),
        GamedataSoundReferencesVerificationResult {
          checked_references_count: 1,
          findings: vec![finding.clone()],
          invalid_references_count: 1,
        },
      )),
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
    let result: GamedataSoundsVerificationResult = GamedataSoundsVerificationResult::new(
      0,
      GamedataSoundFilesVerificationResult::default(),
      GamedataSoundReferencesVerificationResult {
        checked_references_count: 1,
        invalid_references_count: 1,
        ..Default::default()
      },
    );

    assert_eq!(result.status(), GamedataVerificationStatus::Failed);
  }
}

use std::time::Duration;

use crate::GamedataFindingFactory;
use crate::project::sounds::sound_files_verification_result::GamedataSoundFilesVerificationResult;
use crate::project::sounds::sound_references_verification_result::GamedataSoundReferencesVerificationResult;
use crate::{Finding, GamedataCheckResult, GamedataVerificationStatus};

pub struct GamedataSoundsVerificationResult {
  pub(crate) duration: Duration,
  findings: Vec<Finding>,
  sound_files: GamedataSoundFilesVerificationResult,
  sound_references: GamedataSoundReferencesVerificationResult,
}

impl GamedataSoundsVerificationResult {
  pub(crate) fn new(
    duration: Duration,
    sound_files: GamedataSoundFilesVerificationResult,
    sound_references: GamedataSoundReferencesVerificationResult,
  ) -> Self {
    let mut findings: Vec<Finding> = sound_files
      .findings()
      .iter()
      .chain(sound_references.findings())
      .cloned()
      .collect();

    findings.sort_by(GamedataFindingFactory::cmp_by_asset_path_rule_and_message);

    Self {
      duration,
      findings,
      sound_files,
      sound_references,
    }
  }
}

impl GamedataCheckResult for GamedataSoundsVerificationResult {
  fn duration(&self) -> Option<Duration> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::aggregate([self.sound_files.status(), self.sound_references.status()])
  }

  fn failure_message(&self) -> String {
    format!(
      "{}; {}",
      self.sound_files.failure_message(),
      self.sound_references.failure_message(),
    )
  }

  fn findings(&self) -> &[Finding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use std::time::Duration;

  use super::GamedataSoundsVerificationResult;
  use crate::GamedataFindingFactory;
  use crate::project::sounds::sound_files_verification_result::GamedataSoundFilesVerificationResult;
  use crate::project::sounds::sound_references_verification_result::GamedataSoundReferencesVerificationResult;
  use crate::{
    Finding, GamedataCheckResult, GamedataVerificationReport, GamedataVerificationRule, GamedataVerificationStatus,
    GamedataVerificationType,
  };

  #[test]
  fn exposes_sound_reference_findings_in_sound_reports() {
    let finding: Finding = GamedataFindingFactory::for_asset(
      GamedataVerificationRule::SoundsReferences,
      "configs/ui/game_tutorials.xml",
      "Unknown sound reference: <sound> = video\\missing",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Sounds,
      Ok(GamedataSoundsVerificationResult::new(
        Duration::ZERO,
        GamedataSoundFilesVerificationResult::default(),
        GamedataSoundReferencesVerificationResult {
          checked_references_count: 1,
          findings: vec![finding.clone()],
          invalid_references_count: 1,
        },
      )),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks()[0].findings(), [finding]);
    assert_eq!(
      report.checks()[0].summary(),
      "0/0 sounds valid; 0/1 sound references valid"
    );
  }

  #[test]
  fn fails_when_a_sound_reference_is_invalid() {
    let result: GamedataSoundsVerificationResult = GamedataSoundsVerificationResult::new(
      Duration::ZERO,
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

//! Aggregate result for assembled weather-cycle validation.

use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

/// Counts and duration reported by the weather-cycle check.
#[derive(Default)]
pub struct GamedataWeathersVerificationResult {
  /// Validation duration in milliseconds.
  pub duration: u128,
  /// Number of direct weather-cycle files that were checked.
  pub checked_weather_files_count: u32,
  /// Per-file validation failures collected from weather-cycle checks.
  pub findings: Vec<GamedataVerificationFinding>,
  /// Number of checked weather-cycle files with at least one problem.
  pub invalid_weather_files_count: u32,
}

impl GamedataCheckResult for GamedataWeathersVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    if self.checked_weather_files_count == 0 {
      GamedataVerificationStatus::Failed
    } else {
      GamedataVerificationStatus::from_is_valid(self.invalid_weather_files_count == 0)
    }
  }

  fn failure_message(&self) -> String {
    if self.checked_weather_files_count == 0 {
      String::from("No weather files found")
    } else {
      format!(
        "{}/{} weather files are invalid",
        self.invalid_weather_files_count, self.checked_weather_files_count
      )
    }
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataWeathersVerificationResult;
  use crate::{
    GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationReport,
    GamedataVerificationStatus, GamedataVerificationType,
  };

  #[test]
  fn parsed_and_validated_weather_files_pass() {
    let result: GamedataWeathersVerificationResult = GamedataWeathersVerificationResult {
      checked_weather_files_count: 1,
      ..Default::default()
    };

    assert_eq!(result.status(), GamedataVerificationStatus::Passed);
  }

  #[test]
  fn missing_weather_files_fail() {
    let result: GamedataWeathersVerificationResult = GamedataWeathersVerificationResult::default();

    assert_eq!(result.status(), GamedataVerificationStatus::Failed);
    assert_eq!(result.failure_message(), "No weather files found");
  }

  #[test]
  fn exposes_weather_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      "configs/environment/weathers/test.ltx",
      "Weather [00:00:00] is missing required field [fog_color]",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Weathers,
      Ok(GamedataWeathersVerificationResult {
        checked_weather_files_count: 1,
        findings: vec![finding.clone()],
        invalid_weather_files_count: 1,
        ..Default::default()
      }),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

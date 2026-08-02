//! Aggregate result for assembled weather-cycle validation.

use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

/// Counts and duration reported by the weather-cycle check.
#[derive(Default)]
pub struct GamedataWeathersVerificationResult {
  /// Validation duration in milliseconds.
  pub duration: u128,
  /// Number of direct weather-cycle files that were checked.
  pub checked_weather_files_count: u32,
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
}

#[cfg(test)]
mod tests {
  use super::GamedataWeathersVerificationResult;
  use crate::{GamedataCheckResult, GamedataVerificationStatus};

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
}

use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataWeathersVerificationResult {
  pub duration: u128,
  pub checked_weather_files_count: u32,
  pub invalid_weather_files_count: u32,
}

impl GamedataCheckResult for GamedataWeathersVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    if self.invalid_weather_files_count == 0 {
      GamedataVerificationStatus::Incomplete
    } else {
      GamedataVerificationStatus::Failed
    }
  }

  fn failure_message(&self) -> String {
    if self.invalid_weather_files_count == 0 {
      String::from("Weather validation parses files but does not validate their semantics")
    } else {
      format!(
        "{}/{} weather files failed to parse",
        self.invalid_weather_files_count, self.checked_weather_files_count
      )
    }
  }
}

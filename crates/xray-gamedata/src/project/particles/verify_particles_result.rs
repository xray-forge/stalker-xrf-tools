use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;

#[derive(Default)]
pub struct GamedataParticlesVerificationResult {
  pub duration: u128,
  pub checked_particle_files_count: u32,
  pub invalid_particle_files_count: u32,
}

impl GamedataCheckResult for GamedataParticlesVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_particle_files_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} particle library files are invalid",
      self.invalid_particle_files_count, self.checked_particle_files_count
    )
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataParticlesVerificationResult;
  use crate::GamedataCheckResult;

  #[test]
  fn describes_particle_library_failures() {
    let result = GamedataParticlesVerificationResult {
      checked_particle_files_count: 1,
      invalid_particle_files_count: 1,
      ..Default::default()
    };

    assert_eq!(
      result.failure_message(),
      "1/1 particle library files are invalid"
    );
  }
}

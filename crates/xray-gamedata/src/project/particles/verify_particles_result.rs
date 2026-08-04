use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub struct GamedataParticlesVerificationResult {
  pub duration: u128,
  pub checked_particle_files_count: u32,
  pub findings: Vec<GamedataVerificationFinding>,
  pub invalid_particle_files_count: u32,
}

impl GamedataCheckResult for GamedataParticlesVerificationResult {
  fn duration(&self) -> Option<u128> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_particle_files_count == 0)
  }

  fn failure_message(&self) -> String {
    format!(
      "{}/{} particle library files valid",
      self.checked_particle_files_count - self.invalid_particle_files_count,
      self.checked_particle_files_count
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
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

    assert_eq!(result.failure_message(), "0/1 particle library files valid");
  }
}

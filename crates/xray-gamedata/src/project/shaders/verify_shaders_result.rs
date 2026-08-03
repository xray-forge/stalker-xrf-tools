use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

pub struct GamedataShadersVerificationResult;

impl GamedataCheckResult for GamedataShadersVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::Incomplete
  }

  fn failure_message(&self) -> String {
    String::from("Renderer shader validation is not implemented")
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &[]
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataShadersVerificationResult;
  use crate::{GamedataCheckResult, GamedataVerificationStatus};

  #[test]
  fn remains_incomplete_until_renderer_shader_validation_exists() {
    assert_eq!(
      GamedataShadersVerificationResult.status(),
      GamedataVerificationStatus::Incomplete
    );
  }
}

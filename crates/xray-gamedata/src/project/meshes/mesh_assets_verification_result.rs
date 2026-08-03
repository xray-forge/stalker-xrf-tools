use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

#[derive(Default)]
pub(crate) struct GamedataMeshAssetsVerificationResult {
  pub(crate) checked_meshes_count: u32,
  pub(crate) findings: Vec<GamedataVerificationFinding>,
  pub(crate) invalid_meshes_count: u32,
  pub(crate) is_skipped: bool,
}

impl GamedataCheckResult for GamedataMeshAssetsVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    if self.is_skipped {
      GamedataVerificationStatus::Skipped
    } else {
      GamedataVerificationStatus::from_is_valid(self.invalid_meshes_count == 0)
    }
  }

  fn failure_message(&self) -> String {
    if self.is_skipped {
      String::from("Mesh assets were not checked because shaders.xr is invalid")
    } else {
      format!(
        "{}/{} meshes valid",
        self.checked_meshes_count - self.invalid_meshes_count,
        self.checked_meshes_count
      )
    }
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

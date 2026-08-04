use crate::project::meshes::mesh_assets_verification_result::GamedataMeshAssetsVerificationResult;
use crate::project::meshes::shader_library_verification_result::GamedataShaderLibraryVerificationResult;
use crate::{GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationStatus};

pub struct GamedataMeshesVerificationResult {
  pub duration: u128,
  findings: Vec<GamedataVerificationFinding>,
  mesh_assets: GamedataMeshAssetsVerificationResult,
  shader_library: GamedataShaderLibraryVerificationResult,
}

impl GamedataMeshesVerificationResult {
  pub(crate) fn from_checks(
    duration: u128,
    shader_library: GamedataShaderLibraryVerificationResult,
    mesh_assets: GamedataMeshAssetsVerificationResult,
  ) -> Self {
    let mut findings = shader_library.findings().to_vec();

    findings.extend_from_slice(mesh_assets.findings());

    Self {
      duration,
      findings,
      mesh_assets,
      shader_library,
    }
  }
}

impl GamedataCheckResult for GamedataMeshesVerificationResult {
  fn duration(&self) -> Option<u128> {
    Some(self.duration)
  }

  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::aggregate([self.shader_library.status(), self.mesh_assets.status()])
  }

  fn failure_message(&self) -> String {
    format!(
      "{}; {}",
      self.shader_library.failure_message(),
      self.mesh_assets.failure_message()
    )
  }

  fn findings(&self) -> &[GamedataVerificationFinding] {
    &self.findings
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataMeshesVerificationResult;
  use crate::project::meshes::mesh_assets_verification_result::GamedataMeshAssetsVerificationResult;
  use crate::project::meshes::shader_library_verification_result::GamedataShaderLibraryVerificationResult;
  use crate::{
    GamedataVerificationFinding, GamedataVerificationReport, GamedataVerificationStatus,
    GamedataVerificationType,
  };

  #[test]
  fn exposes_mesh_findings_in_reports() {
    let finding: GamedataVerificationFinding = GamedataVerificationFinding::for_asset(
      "meshes/test.ogf",
      "Mesh references missing texture 'textures/test'",
    );
    let mut report: GamedataVerificationReport = GamedataVerificationReport::default();

    report.add_check(
      GamedataVerificationType::Meshes,
      Ok(GamedataMeshesVerificationResult::from_checks(
        0,
        GamedataShaderLibraryVerificationResult::passed(Default::default()),
        GamedataMeshAssetsVerificationResult {
          checked_meshes_count: 1,
          findings: vec![finding.clone()],
          invalid_meshes_count: 1,
          ..Default::default()
        },
      )),
    );

    assert_eq!(report.status(), GamedataVerificationStatus::Failed);
    assert_eq!(report.checks[0].findings, vec![finding]);
  }
}

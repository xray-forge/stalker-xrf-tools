use crate::project::meshes::mesh_assets_verification_result::GamedataMeshAssetsVerificationResult;
use crate::project::meshes::mesh_assets_verifier::MeshAssetsVerifier;
use crate::project::meshes::shader_library_verifier::ShaderLibraryVerifier;
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::{GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};
use std::time::Instant;
use xray_error::XRayResult;

pub(crate) struct MeshesVerifier<'a> {
  options: &'a GamedataProjectVerifyOptions,
  project: &'a GamedataProject,
}

impl<'a> MeshesVerifier<'a> {
  pub(crate) fn new(
    project: &'a GamedataProject,
    options: &'a GamedataProjectVerifyOptions,
  ) -> Self {
    Self { options, project }
  }

  pub(crate) fn verify(&self) -> XRayResult<GamedataMeshesVerificationResult> {
    xray_output::heading!(self.options.output, "Verify meshes:");

    let started_at: Instant = Instant::now();

    let shader_library = ShaderLibraryVerifier::new(self.project).verify();
    let mesh_assets: GamedataMeshAssetsVerificationResult = match shader_library.library() {
      Some(shader_library) => {
        MeshAssetsVerifier::new(self.project, self.options, shader_library).verify()?
      }
      None => GamedataMeshAssetsVerificationResult {
        is_skipped: true,
        ..Default::default()
      },
    };

    let result = GamedataMeshesVerificationResult::from_checks(
      started_at.elapsed(),
      shader_library,
      mesh_assets,
    );

    xray_output::info!(
      self.options.output,
      "Verified gamedata meshes in {} sec, {}",
      result.duration.as_secs_f64(),
      result.failure_message()
    );

    Ok(result)
  }
}

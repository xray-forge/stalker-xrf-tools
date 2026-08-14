use xrf_error::XrfResult;

use crate::project::meshes::meshes_verifier::MeshesVerifier;
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};

impl GamedataProject {
  pub fn verify_meshes(&self, options: &GamedataProjectVerifyOptions) -> XrfResult<GamedataMeshesVerificationResult> {
    MeshesVerifier::new(self, options).verify()
  }
}

use xrf_error::XRayResult;

use crate::project::shaders::shaders_verifier::ShadersVerifier;
use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};

impl GamedataProject {
  pub fn verify_shaders(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataShadersVerificationResult> {
    Ok(ShadersVerifier::new(self.root().join("shaders"), options).verify())
  }
}

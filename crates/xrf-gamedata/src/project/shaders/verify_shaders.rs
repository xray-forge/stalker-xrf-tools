use xrf_error::XrfResult;

use crate::project::shaders::shaders_verifier::ShadersVerifier;
use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};

impl GamedataProject {
  pub fn verify_shaders(&self, options: &GamedataProjectVerifyOptions) -> XrfResult<GamedataShadersVerificationResult> {
    Ok(ShadersVerifier::new(self.vfs(), self.scope(), options).verify())
  }
}

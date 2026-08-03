use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_shaders(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataShadersVerificationResult> {
    let _ = options;

    Ok(GamedataShadersVerificationResult)
  }
}

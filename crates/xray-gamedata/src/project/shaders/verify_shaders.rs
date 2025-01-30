use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_shaders(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataShadersVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata shaders (todo):".green(),);
    }

    Ok(GamedataShadersVerificationResult {})
  }
}

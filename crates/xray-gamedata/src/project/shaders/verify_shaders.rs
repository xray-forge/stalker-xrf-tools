use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_shaders(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataShadersVerificationResult> {
    let result = GamedataShadersVerificationResult::default();

    if options.is_logging_enabled() {
      println!("{}", "Verify shaders:".green());
      println!("  - {}: {}", result.status(), result.failure_message());
    }

    Ok(result)
  }
}

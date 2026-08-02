use crate::project::levels::verify_levels_result::GamedataLevelVerificationResult;
use crate::{GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_levels(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataLevelVerificationResult> {
    let result = GamedataLevelVerificationResult::default();

    if options.is_logging_enabled() {
      println!("{}", "Verify levels:".green());
      println!("  - {}: {}", result.status(), result.failure_message());
    }

    // todo: For now just mark files as used.

    Ok(result)
  }
}

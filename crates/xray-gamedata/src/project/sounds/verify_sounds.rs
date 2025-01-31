use crate::project::sounds::verify_sounds_result::GamedataSoundsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_sounds(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataSoundsVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify sounds:".green());
      println!("  - todo")
    }

    Ok(GamedataSoundsVerificationResult {})
  }
}

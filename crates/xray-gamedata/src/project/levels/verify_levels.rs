use crate::project::levels::verify_levels_result::GamedataLevelVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_levels(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataLevelVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata levels (todo):".green(),);
    }

    // todo: For now just mark files as used.

    Ok(GamedataLevelVerificationResult {})
  }
}

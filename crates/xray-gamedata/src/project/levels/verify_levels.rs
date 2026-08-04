use crate::project::levels::verify_levels_result::GamedataLevelVerificationResult;
use crate::{GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_levels(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataLevelVerificationResult> {
    let result = GamedataLevelVerificationResult::default();

    xray_output::heading!(options.output, "Verify levels:");
    xray_output::info!(
      options.output,
      "  - {}: {}",
      result.status(),
      result.failure_message()
    );

    // todo: For now just mark files as used.

    Ok(result)
  }
}

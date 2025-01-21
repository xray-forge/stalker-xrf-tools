use crate::project::sounds::verify_sounds_result::GamedataSoundsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_sounds(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataSoundsVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata sounds (todo):".green());
    }

    Ok(GamedataSoundsVerificationResult {})
  }
}

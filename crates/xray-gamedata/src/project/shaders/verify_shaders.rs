use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_shaders(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataShadersVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata shaders (todo):".green(),);
    }

    Ok(GamedataShadersVerificationResult {})
  }
}

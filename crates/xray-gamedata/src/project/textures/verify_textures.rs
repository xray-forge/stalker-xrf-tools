use crate::project::textures::verify_textures_result::GamedataTexturesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_textures(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataTexturesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata textures (todo):".green());
    }

    Ok(GamedataTexturesVerificationResult {})
  }
}

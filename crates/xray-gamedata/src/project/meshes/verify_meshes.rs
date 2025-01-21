use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_meshes(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataMeshesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata meshes (todo):".green());
    }

    // todo: Verify linked textures.

    Ok(GamedataMeshesVerificationResult {})
  }
}

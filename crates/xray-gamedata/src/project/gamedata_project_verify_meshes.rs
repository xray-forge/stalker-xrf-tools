use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_meshes(&self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata meshes (todo):".green());
    }

    // todo: Verify linked textures.

    Ok(())
  }
}

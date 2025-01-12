use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_textures(&self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata textures (todo):".green());
    }

    Ok(())
  }
}

use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_meshes(&self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    log::info!("Verify gamedata meshes");

    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata meshes".green());
    }

    Ok(())
  }
}

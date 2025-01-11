use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_sounds(&self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    log::info!("Verify gamedata sounds");

    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata sounds".green());
    }

    Ok(())
  }
}

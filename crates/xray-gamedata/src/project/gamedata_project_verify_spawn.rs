use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_spawns(&self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    log::info!("Verify gamedata spawns");

    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata spawns".green());
    }

    Ok(())
  }
}

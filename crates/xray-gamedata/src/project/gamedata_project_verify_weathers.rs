use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_weathers(&mut self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    log::info!("Verify gamedata weathers");

    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata weathers:".green(),);
    }

    Ok(())
  }
}

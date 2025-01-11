use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_resources_usage(&self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    log::info!("Verify gamedata resources usage");

    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata resources usage".green());

      println!(
        "Resources usage: {} / {}",
        self.assets.values().filter(|it| it.hits > 0).count(),
        self.assets.len()
      );
    }

    Ok(())
  }
}

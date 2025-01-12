use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_levels(&mut self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata levels (todo):".green(),);
    }

    // todo: For now just mark files as used.

    Ok(())
  }
}

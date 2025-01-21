use crate::project::resources::verify_resources_result::GamedataResourcesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_resources_usage(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataResourcesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata resources usage:".green());

      println!(
        "Confirmed resources usage: {} / {}",
        self.assets.values().filter(|it| it.hits > 0).count(),
        self.assets.len()
      );
    }

    if options.is_verbose_logging_enabled() {
      for (_descriptor, descriptor) in &self.assets {
        if descriptor.hits == 0 {
          // println!("Unused resource: -{}", path.red());
        }
      }
    }

    Ok(GamedataResourcesVerificationResult {})
  }
}

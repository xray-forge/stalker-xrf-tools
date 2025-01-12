use crate::project::gamedata_asset_descriptor::GamedataAssetType;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_animations(&mut self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata animations (todo):".green(),);
    }

    // todo: For now just mark files as used.
    for (_, descriptor) in &mut self.assets {
      if descriptor.asset_type == GamedataAssetType::Anm {
        descriptor.hits += 1;
      }
    }

    Ok(())
  }
}

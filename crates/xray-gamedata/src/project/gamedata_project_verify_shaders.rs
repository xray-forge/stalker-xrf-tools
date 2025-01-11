use crate::project::gamedata_asset_descriptor::GamedataAssetType;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_shaders(&mut self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    log::info!("Verify gamedata shaders");

    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata shaders:".green(),);
    }

    // todo: For now just mark files as used.
    for (_, descriptor) in &mut self.assets {
      if descriptor.extension == GamedataAssetType::Shader {
        descriptor.hits += 1;
      }
    }

    Ok(())
  }
}

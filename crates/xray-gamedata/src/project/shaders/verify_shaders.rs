use crate::project::gamedata_asset_descriptor::GamedataAssetType;
use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;

impl GamedataProject {
  pub fn verify_shaders(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataShadersVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata shaders (todo):".green(),);
    }

    // todo: For now just mark files as used.
    for (_, descriptor) in &mut self.assets {
      if descriptor.asset_type == GamedataAssetType::Shader {
        descriptor.hits += 1;
      }
    }

    Ok(GamedataShadersVerificationResult {})
  }
}

use crate::types::GamedataResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationResult};
use std::time::Instant;

impl GamedataProject {
  pub fn verify(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataVerificationResult> {
    if options.is_logging_enabled() {
      println!(
        "Verifying gamedata project: {:?} | {:?}",
        self.roots, self.configs
      );
    }

    let started_at: Instant = Instant::now();

    let mut result: GamedataVerificationResult = GamedataVerificationResult {
      duration: 0,
      format_result: self.verify_ltx_format(options),
      schemes_result: self.verify_ltx_schemes(options),
      spawns_result: self.verify_spawns(options),
      weapons_result: self.verify_ltx_weapons(options),
      meshes_result: self.verify_meshes(options),
      animations_result: self.verify_animations(options),
      textures_result: self.verify_textures(options),
      sounds_result: self.verify_sounds(options),
      levels_result: self.verify_levels(options),
      weathers_result: self.verify_weathers(options),
      scripts_result: self.verify_scripts(options),
      shaders_result: self.verify_shaders(options),
      resources_result: self.verify_resources_usage(options),
      // todo: Verify lua syntax and format with scripts check.
    };

    result.duration = started_at.elapsed().as_millis();

    Ok(result)
  }
}

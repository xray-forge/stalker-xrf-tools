use crate::types::GamedataResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationResult};
use std::time::Instant;
use xray_utils::path_vec_to_string;

impl GamedataProject {
  pub fn verify(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataVerificationResult> {
    if options.is_logging_enabled() {
      println!(
        "Verifying gamedata project: [{}] | {}",
        path_vec_to_string(&self.roots),
        self.configs.display()
      );
    }

    let started_at: Instant = Instant::now();

    let mut result: GamedataVerificationResult = GamedataVerificationResult {
      duration: 0,
      animations_result: self.verify_animations(options),
      format_result: self.verify_ltx_format(options),
      levels_result: self.verify_levels(options),
      meshes_result: self.verify_meshes(options),
      particles_result: self.verify_particles(options),
      schemes_result: self.verify_ltx_schemes(options),
      scripts_result: self.verify_scripts(options),
      shaders_result: self.verify_shaders(options),
      sounds_result: self.verify_sounds(options),
      spawns_result: self.verify_spawns(options),
      textures_result: self.verify_textures(options),
      weapons_result: self.verify_ltx_weapons(options),
      weathers_result: self.verify_weathers(options),
      // todo: Verify lua syntax and format with scripts check.
    };

    result.duration = started_at.elapsed().as_millis();

    Ok(result)
  }
}

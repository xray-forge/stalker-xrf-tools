use crate::{
  GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationResult,
  GamedataVerificationType,
};
use std::time::Instant;
use xray_error::{XRayError, XRayResult};
use xray_utils::path_vec_to_string;

impl GamedataProject {
  pub fn verify(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataVerificationResult> {
    if options.checks.is_empty() {
      return Err(XRayError::new_unexpected_error(
        "No gamedata checks to perform provided",
      ));
    }

    if options.is_logging_enabled() {
      println!(
        "Verifying gamedata project: [{}] | {}",
        path_vec_to_string(&self.roots),
        self.configs.display()
      );

      println!(
        "Verifying modules: \n  -{}",
        options
          .checks
          .iter()
          .map(GamedataVerificationType::to_string)
          .collect::<Vec<_>>()
          .join("\n  -")
      );
    }

    let started_at: Instant = Instant::now();

    let mut result: GamedataVerificationResult = GamedataVerificationResult {
      duration: 0,
      animations_result: GamedataVerificationType::Animations
        .contains_and_then(&options.checks, || self.verify_animations(options)),
      ltx_result: GamedataVerificationType::Ltx
        .contains_and_then(&options.checks, || self.verify_ltx(options)),
      levels_result: GamedataVerificationType::Levels
        .contains_and_then(&options.checks, || self.verify_levels(options)),
      meshes_result: GamedataVerificationType::Meshes
        .contains_and_then(&options.checks, || self.verify_meshes(options)),
      particles_result: GamedataVerificationType::Particles
        .contains_and_then(&options.checks, || self.verify_particles(options)),
      scripts_result: GamedataVerificationType::Scripts
        .contains_and_then(&options.checks, || self.verify_scripts(options)),
      shaders_result: GamedataVerificationType::Shaders
        .contains_and_then(&options.checks, || self.verify_shaders(options)),
      sounds_result: GamedataVerificationType::Sounds
        .contains_and_then(&options.checks, || self.verify_sounds(options)),
      spawns_result: GamedataVerificationType::Spawns
        .contains_and_then(&options.checks, || self.verify_spawns(options)),
      textures_result: GamedataVerificationType::Textures
        .contains_and_then(&options.checks, || self.verify_textures(options)),
      weapons_result: GamedataVerificationType::Weapons
        .contains_and_then(&options.checks, || self.verify_weapons(options)),
      weathers_result: GamedataVerificationType::Weathers
        .contains_and_then(&options.checks, || self.verify_weathers(options)),
    };

    result.duration = started_at.elapsed().as_millis();

    Ok(result)
  }
}

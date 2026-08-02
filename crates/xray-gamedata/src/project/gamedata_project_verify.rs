use crate::{
  GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationReport,
  GamedataVerificationType,
};
use std::time::Instant;
use xray_error::{XRayError, XRayResult};

impl GamedataProject {
  pub fn verify(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataVerificationReport> {
    if options.checks.is_empty() {
      return Err(XRayError::new_unexpected_error(
        "No gamedata checks to perform provided",
      ));
    }

    if options.is_logging_enabled() {
      println!("Verifying gamedata project: {}", self.root.display());

      println!(
        "Verifying modules: \n  -{}",
        options
          .checks
          .iter()
          .map(GamedataVerificationType::to_string)
          .collect::<Vec<_>>()
          .join("\n  -")
      );

      println!();
    }

    let started_at: Instant = Instant::now();

    let mut result: GamedataVerificationReport = GamedataVerificationReport::default();

    if options
      .checks
      .contains(&GamedataVerificationType::Animations)
    {
      result.add_check(
        GamedataVerificationType::Animations,
        self.verify_animations(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Ltx) {
      result.add_check(GamedataVerificationType::Ltx, self.verify_ltx(options));
    }

    if options.checks.contains(&GamedataVerificationType::Levels) {
      result.add_check(
        GamedataVerificationType::Levels,
        self.verify_levels(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Meshes) {
      result.add_check(
        GamedataVerificationType::Meshes,
        self.verify_meshes(options),
      );
    }

    if options
      .checks
      .contains(&GamedataVerificationType::Particles)
    {
      result.add_check(
        GamedataVerificationType::Particles,
        self.verify_particles(options),
      );
    }

    if options
      .checks
      .contains(&GamedataVerificationType::ParticlesUsage)
    {
      result.add_check(
        GamedataVerificationType::ParticlesUsage,
        self.verify_particles_usage(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Scripts) {
      result.add_check(
        GamedataVerificationType::Scripts,
        self.verify_scripts(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Shaders) {
      result.add_check(
        GamedataVerificationType::Shaders,
        self.verify_shaders(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Sounds) {
      result.add_check(
        GamedataVerificationType::Sounds,
        self.verify_sounds(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Spawns) {
      result.add_check(
        GamedataVerificationType::Spawns,
        self.verify_spawns(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Textures) {
      result.add_check(
        GamedataVerificationType::Textures,
        self.verify_textures(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Weapons) {
      result.add_check(
        GamedataVerificationType::Weapons,
        self.verify_weapons(options),
      );
    }

    if options.checks.contains(&GamedataVerificationType::Weathers) {
      result.add_check(
        GamedataVerificationType::Weathers,
        self.verify_weathers(options),
      );
    }

    result.duration = started_at.elapsed().as_millis();

    Ok(result)
  }
}

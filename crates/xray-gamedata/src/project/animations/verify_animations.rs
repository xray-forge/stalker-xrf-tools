use crate::project::animations::player_hud_animations_verifier::PlayerHudAnimationsVerifier;
use crate::project::animations::verify_animations_result::GamedataAnimationsVerificationResult;
use crate::{GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use std::time::Instant;
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_animations(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataAnimationsVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify animations:".green());
    }

    let started_at: Instant = Instant::now();

    let player_hud_animations = PlayerHudAnimationsVerifier::new(self, options).verify()?;

    let result: GamedataAnimationsVerificationResult = GamedataAnimationsVerificationResult {
      duration: started_at.elapsed(),
      findings: player_hud_animations.findings().to_vec(),
      player_hud_animations,
    };

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata animations in {} sec, {}",
        result.duration.as_secs_f64(),
        result.failure_message()
      );
    }

    Ok(result)
  }
}

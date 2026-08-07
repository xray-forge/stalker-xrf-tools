use crate::project::animations::hud_item_animations_verification_result::GamedataHudItemAnimationsVerificationResult;
use crate::project::animations::hud_item_animations_verifier::HudItemAnimationsVerifier;
use crate::project::animations::hud_motion_collisions_verification_result::GamedataHudMotionCollisionsVerificationResult;
use crate::project::animations::hud_motion_collisions_verifier::HudMotionCollisionsVerifier;
use crate::project::animations::player_hud_animations_verification_result::GamedataPlayerHudAnimationsVerificationResult;
use crate::project::animations::player_hud_animations_verifier::PlayerHudAnimationsVerifier;
use crate::project::animations::verify_animations_result::GamedataAnimationsVerificationResult;
use crate::{Finding, GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};
use std::time::Instant;
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_animations(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataAnimationsVerificationResult> {
    xray_output::heading!(options.output, "Verify animations:");

    let started_at: Instant = Instant::now();

    let player_hud_animations: GamedataPlayerHudAnimationsVerificationResult =
      PlayerHudAnimationsVerifier::new(self, options).verify()?;
    let hud_item_animations: GamedataHudItemAnimationsVerificationResult =
      HudItemAnimationsVerifier::new(self, options).verify()?;
    let hud_motion_collisions: GamedataHudMotionCollisionsVerificationResult =
      HudMotionCollisionsVerifier::new(self, options).verify()?;

    let mut findings: Vec<Finding> = player_hud_animations.findings().to_vec();

    findings.extend(hud_item_animations.findings().to_vec());
    findings.extend(hud_motion_collisions.findings().to_vec());

    let result: GamedataAnimationsVerificationResult = GamedataAnimationsVerificationResult {
      duration: started_at.elapsed(),
      findings,
      hud_item_animations,
      hud_motion_collisions,
      player_hud_animations,
    };

    xray_output::info!(
      options.output,
      "Verified gamedata animations in {} sec, {}",
      result.duration.as_secs_f64(),
      result.failure_message()
    );

    Ok(result)
  }
}

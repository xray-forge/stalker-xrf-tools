use xray_error::XRayResult;

use crate::project::levels::levels_verifier::LevelsVerifier;
use crate::project::levels::verify_levels_result::GamedataLevelsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};

impl GamedataProject {
  // todo: Level bundles are the largest asset family in gamedata, so they are the natural first
  //   producer of asset usage data. Marking consumed assets is deliberately not done here - it is a
  //   whole suite concern that needs a usage marking API on the asset index and a policy for mods
  //   that ship spare assets, and deciding it from inside one check would fix the shape for all.
  pub fn verify_levels(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataLevelsVerificationResult> {
    LevelsVerifier::new(self, options).verify()
  }
}

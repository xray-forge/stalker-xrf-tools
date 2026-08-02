use crate::project::sounds::sounds_verification_result::GamedataSoundsVerificationResult;
use crate::project::sounds::sounds_verifier::SoundsVerifier;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_sounds(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataSoundsVerificationResult> {
    SoundsVerifier::new(self, options).verify()
  }
}

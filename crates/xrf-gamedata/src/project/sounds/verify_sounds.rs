use xrf_error::XrfResult;

use crate::project::sounds::sounds_verification_result::GamedataSoundsVerificationResult;
use crate::project::sounds::sounds_verifier::SoundsVerifier;
use crate::{GamedataProject, GamedataProjectVerifyOptions};

impl GamedataProject {
  pub fn verify_sounds(&self, options: &GamedataProjectVerifyOptions) -> XrfResult<GamedataSoundsVerificationResult> {
    SoundsVerifier::new(self, options).verify()
  }
}

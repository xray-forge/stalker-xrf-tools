use crate::asset::asset_type::AssetType;
use crate::project::sounds::sound_files_verifier::SoundFilesVerifier;
use crate::project::sounds::sound_references_verifier::SoundReferencesVerifier;
use crate::project::sounds::sounds_verification_result::GamedataSoundsVerificationResult;
use crate::{GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use std::time::Instant;
use xray_error::XRayResult;

pub(crate) struct SoundsVerifier<'a> {
  options: &'a GamedataProjectVerifyOptions,
  project: &'a GamedataProject,
}

impl<'a> SoundsVerifier<'a> {
  pub(crate) fn new(
    project: &'a GamedataProject,
    options: &'a GamedataProjectVerifyOptions,
  ) -> Self {
    Self { options, project }
  }

  pub(crate) fn verify(&self) -> XRayResult<GamedataSoundsVerificationResult> {
    if self.options.is_logging_enabled() {
      println!("{}", "Verify sounds:".green());
    }

    let started_at: Instant = Instant::now();

    let sound_paths: Vec<String> = self.project.get_all_asset_paths_by_type(AssetType::Ogg);
    let sound_files = SoundFilesVerifier::new(self.project, self.options, &sound_paths).verify()?;
    let sound_references =
      SoundReferencesVerifier::new(self.project, self.options, &sound_paths).verify()?;

    let result: GamedataSoundsVerificationResult = GamedataSoundsVerificationResult::new(
      started_at.elapsed().as_millis(),
      sound_files,
      sound_references,
    );

    if self.options.is_logging_enabled() {
      println!(
        "Verified gamedata sounds in {} sec, {}",
        (result.duration as f64) / 1000.0,
        result.failure_message()
      );
    }

    Ok(result)
  }
}

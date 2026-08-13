use std::time::Instant;

use xrf_assets::XrayAssetType as AssetType;
use xrf_error::XRayResult;

use crate::project::sounds::sound_files_verifier::SoundFilesVerifier;
use crate::project::sounds::sound_references_verifier::SoundReferencesVerifier;
use crate::project::sounds::sounds_verification_result::GamedataSoundsVerificationResult;
use crate::{GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};

pub(crate) struct SoundsVerifier<'a> {
  options: &'a GamedataProjectVerifyOptions,
  project: &'a GamedataProject,
}

impl<'a> SoundsVerifier<'a> {
  pub(crate) fn new(project: &'a GamedataProject, options: &'a GamedataProjectVerifyOptions) -> Self {
    Self { options, project }
  }

  pub(crate) fn verify(&self) -> XRayResult<GamedataSoundsVerificationResult> {
    xrf_output::heading!(self.options.output, "Verify sounds:");

    let started_at: Instant = Instant::now();

    let sound_paths: Vec<String> = self
      .project
      .assets
      .with_type(AssetType::Ogg)
      .map(|asset| asset.logical_path().to_string())
      .collect();

    let sound_files = SoundFilesVerifier::new(self.project, self.options, &sound_paths).verify()?;
    let sound_references = SoundReferencesVerifier::new(self.project, self.options, &sound_paths).verify()?;

    let result: GamedataSoundsVerificationResult =
      GamedataSoundsVerificationResult::new(started_at.elapsed(), sound_files, sound_references);

    xrf_output::info!(
      self.options.output,
      "Verified gamedata sounds in {} sec, {}",
      result.duration.as_secs_f64(),
      result.failure_message()
    );

    Ok(result)
  }
}

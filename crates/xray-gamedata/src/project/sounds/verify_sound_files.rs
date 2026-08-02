use crate::project::sounds::verify_sound_files_result::GamedataSoundFilesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding};
use rayon::prelude::*;
use std::path::Path;
use xray_error::XRayResult;
use xray_sound::SoundFile;

pub(crate) struct SoundFilesVerifier<'a> {
  options: &'a GamedataProjectVerifyOptions,
  project: &'a GamedataProject,
  sound_paths: &'a [String],
}

impl<'a> SoundFilesVerifier<'a> {
  pub(crate) fn new(
    project: &'a GamedataProject,
    options: &'a GamedataProjectVerifyOptions,
    sound_paths: &'a [String],
  ) -> Self {
    Self {
      options,
      project,
      sound_paths,
    }
  }

  pub(crate) fn verify(&self) -> XRayResult<GamedataSoundFilesVerificationResult> {
    let mut findings: Vec<GamedataVerificationFinding> = self
      .sound_paths
      .par_iter()
      .filter_map(|relative_path| {
        if self.options.is_verbose_logging_enabled() {
          println!("Verify sound: {relative_path}");
        }

        let Some(path) = self.project.get_absolute_asset_path(relative_path) else {
          return Some(GamedataVerificationFinding::for_asset_in_rule(
            "sounds.files",
            Path::new(relative_path),
            "Sound path was not found in gamedata roots",
          ));
        };

        let sound: XRayResult<SoundFile> = if self.options.is_strict {
          SoundFile::read_strictly_from_path(&path)
        } else {
          SoundFile::read_from_path(&path)
        };

        sound.err().map(|error| {
          if self.options.is_logging_enabled() {
            eprintln!("Sound is not valid: {} - {error}", path.display());
          }

          GamedataVerificationFinding::for_asset_in_rule("sounds.files", path, error.to_string())
        })
      })
      .collect();

    findings.sort_by(|left, right| {
      left
        .asset_path
        .cmp(&right.asset_path)
        .then_with(|| left.message.cmp(&right.message))
    });

    Ok(GamedataSoundFilesVerificationResult {
      checked_sounds_count: self.sound_paths.len() as u32,
      invalid_sounds_count: findings.len() as u32,
      findings,
    })
  }
}

use crate::asset::asset_type::AssetType;
use crate::project::sounds::verify_sounds_result::GamedataSoundsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding};
use colored::Colorize;
use rayon::prelude::*;
use std::path::Path;
use std::time::Instant;
use xray_error::XRayResult;
use xray_sound::SoundFile;

impl GamedataProject {
  pub fn verify_sounds(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataSoundsVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify sounds:".green());
    }

    let started_at: Instant = Instant::now();
    let sound_paths: Vec<String> = self.get_all_asset_paths_by_type(AssetType::Ogg);
    let mut findings: Vec<GamedataVerificationFinding> = sound_paths
      .par_iter()
      .filter_map(|relative_path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify sound: {relative_path}");
        }

        let Some(path) = self.get_absolute_asset_path(relative_path) else {
          return Some(GamedataVerificationFinding::for_asset(
            Path::new(relative_path),
            "Sound path was not found in gamedata roots",
          ));
        };

        let sound: XRayResult<SoundFile> = if options.is_strict {
          SoundFile::read_strictly_from_path(&path)
        } else {
          SoundFile::read_from_path(&path)
        };

        sound.err().map(|error| {
          if options.is_logging_enabled() {
            eprintln!("Sound is not valid: {} - {error}", path.display());
          }

          GamedataVerificationFinding::for_asset(path, error.to_string())
        })
      })
      .collect();

    findings.sort_by(|left, right| {
      left
        .asset_path
        .cmp(&right.asset_path)
        .then_with(|| left.message.cmp(&right.message))
    });

    let result: GamedataSoundsVerificationResult = GamedataSoundsVerificationResult {
      duration: started_at.elapsed().as_millis(),
      invalid_sounds_count: findings.len() as u32,
      checked_sounds_count: sound_paths.len() as u32,
      findings,
    };

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata sounds in {} sec, {}/{} valid",
        (result.duration as f64) / 1000.0,
        result.checked_sounds_count - result.invalid_sounds_count,
        result.checked_sounds_count
      );
    }

    Ok(result)
  }
}

use crate::asset::asset_type::AssetType;
use crate::project::spawns::verify_spawns_result::GamedataSpawnsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use std::path::Path;
use std::time::Instant;
use xray_db::{SpawnFile, XRayByteOrder};
use xray_error::XRayResult;

impl GamedataProject {
  /// Verify spawn files in spawns directories, not levels spawn files.
  pub fn verify_spawns(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataSpawnsVerificationResult> {
    let started_at: Instant = Instant::now();

    let spawn_files: Vec<String> = self
      .assets
      .iter()
      .filter(|(relative_path, descriptor)| {
        descriptor.asset_type == AssetType::Spawn && relative_path.starts_with("spawns")
      })
      .map(|(key, _)| key.clone())
      .collect::<Vec<_>>();

    if options.is_logging_enabled() {
      println!(
        "{} {}",
        "Verify gamedata spawns:".green(),
        spawn_files.len()
      );
    }

    if spawn_files.is_empty() {
      if options.is_logging_enabled() {
        println!("No spawn files found in gamedata root");
      }

      // todo: Verify result struct.

      return Ok(GamedataSpawnsVerificationResult {
        duration: started_at.elapsed().as_millis(),
        total_spawns: 0,
        invalid_spawns: 0,
      });
    }

    let mut total_spawns: u32 = 0;
    let mut invalid_spawns: u32 = 0;

    for relative_path in &spawn_files {
      total_spawns += 1;

      if let Some(spawn_path) = self.get_absolute_asset_path(relative_path) {
        if !self.verify_spawn(options, &spawn_path)? {
          invalid_spawns += 1;
        }
      } else {
        invalid_spawns += 1;
      }
    }

    let duration: u128 = started_at.elapsed().as_millis();

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata spawn files in {} sec, {}/{} are valid",
        (duration as f64) / 1000.0,
        total_spawns - invalid_spawns,
        total_spawns
      );
    }

    Ok(GamedataSpawnsVerificationResult {
      duration,
      total_spawns,
      invalid_spawns,
    })
  }

  pub fn verify_spawn(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> XRayResult<bool> {
    if options.is_verbose_logging_enabled() {
      println!("Verify spawn file: {path:?}");
    }

    match SpawnFile::read_from_path::<XRayByteOrder>(path) {
      Ok(_) => {
        if options.is_verbose_logging_enabled() {
          println!("Verify spawn file: {path:?}");
        }

        Ok(true)
      }
      Err(error) => {
        if options.is_logging_enabled() {
          eprintln!("Spawn file validation failed: {path:?} -> {error:?}");
        }

        Ok(false)
      }
    }
  }
}

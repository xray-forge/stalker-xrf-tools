use crate::asset::asset_type::AssetType;
use crate::project::spawns::verify_spawns_result::GamedataSpawnsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use std::path::Path;
use xray_db::{SpawnFile, XRayByteOrder};

impl GamedataProject {
  /// Verify spawn files in spawns directories, not levels spawn files.
  pub fn verify_spawns(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataSpawnsVerificationResult> {
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
        total_spawns: 0,
        invalid_spawns: 0,
      });
    }

    let mut total_spawns: usize = 0;
    let mut invalid_spawns: usize = 0;

    for relative_path in &spawn_files {
      total_spawns += 1;

      if let Some(spawn_path) = self.get_absolute_asset_path_hit(relative_path) {
        if !self.verify_spawn(options, &spawn_path)? {
          invalid_spawns += 1;
        }
      } else {
        invalid_spawns += 1;
      }
    }

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata spawn files: {}/{}",
        total_spawns - invalid_spawns,
        total_spawns
      );
    }

    Ok(GamedataSpawnsVerificationResult {
      total_spawns: total_spawns as u64,
      invalid_spawns: invalid_spawns as u64,
    })
  }

  pub fn verify_spawn(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> GamedataResult<bool> {
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

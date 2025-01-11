use crate::project::gamedata_asset_descriptor::GamedataAssetExtension;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use std::path::Path;
use xray_db::{SpawnFile, XRayByteOrder};

impl GamedataProject {
  /// Verify spawn files in spawns directories, not levels spawn files.
  pub fn verify_spawns(&mut self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    let spawn_files: Vec<String> = self
      .assets
      .iter()
      .filter(|(relative_path, descriptor)| {
        descriptor.extension == GamedataAssetExtension::Spawn && relative_path.starts_with("spawns")
      })
      .map(|(key, _)| key.clone())
      .collect::<Vec<_>>();

    log::info!("Verify gamedata spawns: {}", spawn_files.len());

    if options.is_logging_enabled() {
      println!(
        "{} {}",
        "Verify gamedata spawns:".green(),
        spawn_files.len()
      );
    }

    if spawn_files.is_empty() {
      log::info!("No spawn files found in gamedata root");

      if options.is_logging_enabled() {
        println!("No spawn files found in gamedata root");
      }

      // todo: Verify result struct.

      return Ok(());
    }

    let mut total_spawns: usize = 0;
    let mut invalid_spawns: usize = 0;

    for relative_path in &spawn_files {
      total_spawns += 1;

      if let Some(spawn_path) = self.get_relative_asset_path(relative_path) {
        if !self.verify_spawn(options, &spawn_path)? {
          invalid_spawns += 1;
        }
      }
    }

    if options.is_logging_enabled() {
      log::info!(
        "Verified gamedata spawn files: {}/{}",
        total_spawns - invalid_spawns,
        total_spawns
      );
    }

    Ok(())
  }

  pub fn verify_spawn(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> GamedataResult<bool> {
    log::info!("Verify gamedata spawn file: {path:?}");

    if options.is_verbose_logging_enabled() {
      println!("Verify spawn file: {path:?}");
    }

    match SpawnFile::read_from_path::<XRayByteOrder>(path) {
      Ok(_) => {
        log::info!("Successfully verified spawn: {path:?}",);

        if options.is_verbose_logging_enabled() {
          println!("Verify spawn file: {path:?}");
        }

        Ok(true)
      }
      Err(error) => {
        log::warn!("Spawn file validation failed: {path:?} -> {error:?}");

        if options.is_logging_enabled() {
          eprintln!("Spawn file validation failed: {path:?} -> {error:?}");
        }

        Ok(false)
      }
    }
  }
}

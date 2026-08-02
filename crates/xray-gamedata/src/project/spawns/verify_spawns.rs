use crate::asset::asset_type::AssetType;
use crate::project::spawns::verify_spawns_result::GamedataSpawnsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding};
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
      println!("{} {}", "Verify spawns:".green(), spawn_files.len());
    }

    if spawn_files.is_empty() {
      if options.is_logging_enabled() {
        println!("No spawn files found in gamedata root");
      }

      // todo: Verify result struct.

      return Ok(GamedataSpawnsVerificationResult {
        duration: started_at.elapsed().as_millis(),
        findings: Vec::new(),
        total_spawns: 0,
        invalid_spawns: 0,
      });
    }

    let mut total_spawns: u32 = 0;
    let mut findings: Vec<GamedataVerificationFinding> = Vec::new();
    let mut invalid_spawns: u32 = 0;

    for relative_path in &spawn_files {
      total_spawns += 1;

      if let Some(spawn_path) = self.get_absolute_asset_path(relative_path) {
        let spawn_findings: Vec<GamedataVerificationFinding> =
          self.verify_spawn_findings(options, &spawn_path);

        if !spawn_findings.is_empty() {
          findings.extend(spawn_findings);
          invalid_spawns += 1;
        }
      } else {
        findings.push(GamedataVerificationFinding::for_asset(
          Path::new(relative_path),
          "Spawn path was not found in gamedata roots",
        ));
        invalid_spawns += 1;
      }
    }

    let duration: u128 = started_at.elapsed().as_millis();

    findings.sort_by(|left, right| {
      left
        .asset_path
        .cmp(&right.asset_path)
        .then_with(|| left.message.cmp(&right.message))
    });

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
      findings,
      total_spawns,
      invalid_spawns,
    })
  }

  pub fn verify_spawn<P: AsRef<Path>>(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &P,
  ) -> XRayResult<bool> {
    Ok(self.verify_spawn_findings(options, path).is_empty())
  }

  fn verify_spawn_findings<P: AsRef<Path>>(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &P,
  ) -> Vec<GamedataVerificationFinding> {
    let file_path: String = path.as_ref().display().to_string();

    if options.is_verbose_logging_enabled() {
      println!("Verify spawn file: {}", file_path);
    }

    match SpawnFile::read_from_path::<XRayByteOrder, P>(path) {
      Ok(_) => {
        if options.is_verbose_logging_enabled() {
          println!("Verify spawn file: {}", file_path);
        }

        Vec::new()
      }
      Err(error) => {
        if options.is_logging_enabled() {
          eprintln!("Spawn file validation failed: {} -> {}", file_path, error);
        }

        vec![GamedataVerificationFinding::for_asset(
          path,
          format!("Failed to read spawn file: {error}"),
        )]
      }
    }
  }
}

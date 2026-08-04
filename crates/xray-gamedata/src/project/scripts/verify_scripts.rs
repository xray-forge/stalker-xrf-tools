use crate::asset::asset_type::AssetType;
use crate::project::scripts::runtime_script::is_runtime_script;
use crate::project::scripts::verify_scripts_result::GamedataScriptsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding};
use colored::Colorize;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use xray_error::{XRayError, XRayResult};
use xray_lua::verify_luajit_script;
use xray_utils::read_as_string_from_w1251_encoded;

impl GamedataProject {
  pub fn verify_scripts(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataScriptsVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify scripts:".green());
    }

    let started_at: Instant = Instant::now();
    let script_paths: Vec<String> = self
      .get_all_asset_paths_by_type(AssetType::Script)
      .into_iter()
      .filter(|path| is_runtime_script(path))
      .collect();

    let checked_scripts_count: u32 = u32::try_from(script_paths.len()).map_err(|_| {
      XRayError::new_verify_error("Script count exceeds the supported result range")
    })?;

    let mut findings: Vec<GamedataVerificationFinding> = script_paths
      .par_iter()
      .filter_map(|relative_path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify script: {relative_path}");
        }

        let Some(path) = self.get_absolute_asset_path(relative_path) else {
          if options.is_logging_enabled() {
            println!("Script path not found: {relative_path}");
          }

          return Some(GamedataVerificationFinding::for_asset(
            Path::new(relative_path),
            "Script path was not found in gamedata roots",
          ));
        };

        match self.verify_script(options, &path) {
          Ok(true) => None,
          Ok(false) => {
            if options.is_logging_enabled() {
              println!("Script is not valid: {}", path.display());
            }

            Some(GamedataVerificationFinding::for_asset(
              &path,
              "LuaJIT parser rejected the script",
            ))
          }
          Err(error) => {
            if options.is_logging_enabled() {
              eprintln!("Script verification failed: {error}");
            }

            Some(GamedataVerificationFinding::for_asset(
              &path,
              error.to_string(),
            ))
          }
        }
      })
      .collect();

    let duration: u128 = started_at.elapsed().as_millis();
    let invalid_scripts_count: u32 = u32::try_from(findings.len()).map_err(|_| {
      XRayError::new_verify_error("Invalid script count exceeds the supported result range")
    })?;

    findings.sort_by(|left, right| left.asset_path.cmp(&right.asset_path));

    if options.is_logging_enabled() {
      if checked_scripts_count > 0 {
        println!(
          "Verified gamedata scripts in {} sec, {}/{} valid",
          (duration as f64) / 1000.0,
          checked_scripts_count - invalid_scripts_count,
          checked_scripts_count
        );
      } else {
        println!(
          "Check gamedata scripts in {} sec, no scripts found",
          (duration as f64) / 1000.0,
        );
      }
    }

    Ok(GamedataScriptsVerificationResult {
      duration,
      checked_scripts_count,
      findings,
      invalid_scripts_count,
    })
  }

  pub fn verify_script(
    &self,
    _options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> XRayResult<bool> {
    let code: String = read_as_string_from_w1251_encoded(&mut File::open(path)?)?;

    verify_luajit_script(&code, path)?;

    Ok(true)
  }
}

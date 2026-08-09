use std::fs::File;
use std::path::Path;
use std::time::{Duration, Instant};

use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use xray_assets::XrayAssetType as AssetType;
use xray_error::{XRayError, XRayResult};
use xray_lua::verify_luajit_script;
use xray_utils::read_as_string_from_w1251_encoded;

use crate::GamedataFindingFactory;
use crate::project::scripts::runtime_script::is_runtime_script;
use crate::project::scripts::verify_scripts_result::GamedataScriptsVerificationResult;
use crate::{Finding, GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationRule};

impl GamedataProject {
  pub fn verify_scripts(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataScriptsVerificationResult> {
    xray_output::heading!(options.output, "Verify scripts:");

    let started_at: Instant = Instant::now();
    let script_paths: Vec<String> = self
      .assets
      .with_type(AssetType::Script)
      .map(|asset| asset.logical_path().to_string())
      .collect::<Vec<_>>()
      .into_iter()
      .filter(|path| is_runtime_script(path))
      .collect();

    let checked_scripts_count: u32 = u32::try_from(script_paths.len())
      .map_err(|_| XRayError::new_verify_error("Script count exceeds the supported result range"))?;

    let mut findings: Vec<Finding> = script_paths
      .par_iter()
      .filter_map(|relative_path| {
        xray_output::verbose!(options.output, "Verify script: {relative_path}");

        let Some(path) = self.assets.absolute_path(relative_path).ok().flatten() else {
          xray_output::info!(options.output, "Script path not found: {relative_path}");

          return Some(GamedataFindingFactory::for_asset(
            GamedataVerificationRule::ScriptsPath,
            Path::new(relative_path),
            "Script path was not found in gamedata roots",
          ));
        };

        match self.verify_script(options, &path) {
          Ok(true) => None,
          Ok(false) => {
            xray_output::info!(options.output, "Script is not valid: {}", path.display());

            Some(GamedataFindingFactory::for_asset(
              GamedataVerificationRule::ScriptsSyntax,
              &path,
              "LuaJIT parser rejected the script",
            ))
          }
          Err(error) => {
            xray_output::error!(options.output, "Script verification failed: {error}");

            Some(GamedataFindingFactory::for_asset(
              GamedataVerificationRule::ScriptsRead,
              &path,
              error.to_string(),
            ))
          }
        }
      })
      .collect();

    let duration: Duration = started_at.elapsed();
    let invalid_scripts_count: u32 = u32::try_from(findings.len())
      .map_err(|_| XRayError::new_verify_error("Invalid script count exceeds the supported result range"))?;

    findings.sort_by(GamedataFindingFactory::cmp_by_asset_path_and_message);

    if checked_scripts_count > 0 {
      xray_output::info!(
        options.output,
        "Verified gamedata scripts in {} sec, {}/{} valid",
        duration.as_secs_f64(),
        checked_scripts_count - invalid_scripts_count,
        checked_scripts_count
      );
    } else {
      xray_output::info!(
        options.output,
        "Check gamedata scripts in {} sec, no scripts found",
        duration.as_secs_f64(),
      );
    }

    Ok(GamedataScriptsVerificationResult {
      duration,
      checked_scripts_count,
      findings,
      invalid_scripts_count,
    })
  }

  pub fn verify_script(&self, _options: &GamedataProjectVerifyOptions, path: &Path) -> XRayResult<bool> {
    let code: String = read_as_string_from_w1251_encoded(&mut File::open(path)?)?;

    verify_luajit_script(&code, path)?;

    Ok(true)
  }
}

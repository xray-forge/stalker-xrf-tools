//! Discovery and aggregate reporting for assembled weather cycles.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Instant;

use xray_error::{XRayError, XRayResult};

use super::verify_weathers_result::GamedataWeathersVerificationResult;
use super::weather_definitions::WeatherDefinitions;
use super::weather_validator::verify_weather_findings_with_definitions;
use crate::GamedataFindingFactory;
use crate::{Finding, GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationRule};

impl GamedataProject {
  /// Verifies every assembled weather cycle under `configs/environment/weathers`.
  ///
  /// Weather definitions are loaded once and reused across all discovered cycle files. A missing
  /// cycle directory, an empty cycle directory, or any invalid cycle produces a failed result.
  pub fn verify_weathers(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataWeathersVerificationResult> {
    xray_output::heading!(options.output, "Verify weathers:");

    let started_at: Instant = Instant::now();

    let weathers_path: String = Path::new("environment")
      .join("weathers")
      .to_str()
      .expect("Expected valid weathers path")
      .to_string();
    let weather_configs: Vec<&PathBuf> = self
      .ltx_project
      .ltx_files
      .iter()
      .filter(|path| {
        path
          .parent()
          .expect("Config parent expected")
          .ends_with(&weathers_path)
      })
      .collect();

    let checked_weather_files_count: u32 = u32::try_from(weather_configs.len()).map_err(|_| {
      XRayError::new_verify_error("Weather config count exceeds the supported result range")
    })?;
    let definitions: WeatherDefinitions = WeatherDefinitions::read(&self.ltx_project.root);
    let mut definition_load_errors: BTreeSet<String> = BTreeSet::new();
    let mut findings: Vec<Finding> = Vec::new();
    let mut invalid_weather_files_count: u32 = 0;

    for weather_config in weather_configs {
      let weather_findings: Vec<Finding> = verify_weather_findings_with_definitions(
        self,
        options,
        weather_config,
        &definitions,
        &mut definition_load_errors,
      )?;

      if !weather_findings.is_empty() {
        findings.extend(weather_findings);
        invalid_weather_files_count += 1;
      }
    }

    if checked_weather_files_count == 0 {
      findings.push(GamedataFindingFactory::without_asset(
        GamedataVerificationRule::WeathersFiles,
        "No weather files found",
      ));
    }

    findings.sort_by(GamedataFindingFactory::cmp_by_asset_path_and_message);

    let duration = started_at.elapsed();

    for error in definition_load_errors {
      options.output.error(error);
    }

    if checked_weather_files_count == 0 {
      xray_output::info!(
        options.output,
        "Checked gamedata weather files in {} sec, no weather files found",
        duration.as_secs_f64()
      );
    } else {
      xray_output::info!(
        options.output,
        "Verified gamedata weather files in {} sec, {}/{} valid",
        duration.as_secs_f64(),
        checked_weather_files_count - invalid_weather_files_count,
        checked_weather_files_count
      );
    }

    Ok(GamedataWeathersVerificationResult {
      duration,
      checked_weather_files_count,
      findings,
      invalid_weather_files_count,
    })
  }
}

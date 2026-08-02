//! Discovery and aggregate reporting for assembled weather cycles.

use super::verify_weathers_result::GamedataWeathersVerificationResult;
use super::weather_definitions::WeatherDefinitions;
use super::weather_validator::verify_weather_with_definitions;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Instant;
use xray_error::{XRayError, XRayResult};

impl GamedataProject {
  /// Verifies every assembled weather cycle under `configs/environment/weathers`.
  ///
  /// Weather definitions are loaded once and reused across all discovered cycle files. A missing
  /// cycle directory, an empty cycle directory, or any invalid cycle produces a failed result.
  pub fn verify_weathers(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataWeathersVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify weathers:".green());
    }

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
    let mut invalid_weather_files_count: u32 = 0;

    for weather_config in weather_configs {
      if !verify_weather_with_definitions(
        self,
        options,
        weather_config,
        &definitions,
        &mut definition_load_errors,
      )? {
        invalid_weather_files_count += 1;
      }
    }

    let duration: u128 = started_at.elapsed().as_millis();

    if options.is_logging_enabled() {
      for error in definition_load_errors {
        eprintln!("{error}");
      }

      if checked_weather_files_count == 0 {
        println!(
          "Checked gamedata weather files in {} sec, no weather files found",
          (duration as f64) / 1000.0
        );
      } else {
        println!(
          "Verified gamedata weather files in {} sec, {}/{} valid",
          (duration as f64) / 1000.0,
          checked_weather_files_count - invalid_weather_files_count,
          checked_weather_files_count
        );
      }
    }

    Ok(GamedataWeathersVerificationResult {
      duration,
      checked_weather_files_count,
      invalid_weather_files_count,
    })
  }
}

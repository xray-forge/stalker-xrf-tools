use crate::project::weathers::verify_weathers_result::GamedataWeathersVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::time::Instant;
use xray_error::XRayResult;
use xray_ltx::Ltx;

impl GamedataProject {
  pub fn verify_weathers(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataWeathersVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify weathers:".green());
    }

    let started_at: Instant = Instant::now();
    let duration: u128 = started_at.elapsed().as_millis();

    let weathers_path: String = Path::new("environment")
      .join("weathers")
      .to_str()
      .expect("Expected valid weathers path")
      .to_string();

    let weather_configs: Vec<&PathBuf> = self
      .ltx_project
      .ltx_files
      .iter()
      .filter(|it| {
        it.parent()
          .expect("Config parent expected")
          .ends_with(&weathers_path)
      })
      .collect::<Vec<_>>();

    if options.is_logging_enabled() {
      println!("{} weather configs to verify", weather_configs.len());
    }

    for weather_config in weather_configs {
      // todo: implement and count
      self.verify_weather(options, weather_config)?;
    }

    Ok(GamedataWeathersVerificationResult { duration })
  }

  pub fn verify_weather(
    &self,
    options: &GamedataProjectVerifyOptions,
    config_path: &Path,
  ) -> XRayResult<bool> {
    let mut is_valid: bool = true;

    match Ltx::read_from_file_full(config_path) {
      Ok(_ltx) => {
        // todo: Verify weather
      }
      Err(error) => {
        if options.is_logging_enabled() {
          eprintln!("Could not open weather LTX: {}", error);
        }

        is_valid = false;
      }
    }

    Ok(is_valid)
  }
}

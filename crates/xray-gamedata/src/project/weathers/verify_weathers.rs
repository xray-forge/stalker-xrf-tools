use crate::project::weathers::verify_weathers_result::GamedataWeathersVerificationResult;
use crate::{GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions};
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

    let checked_weather_files_count: u32 = weather_configs.len() as u32;
    let mut invalid_weather_files_count: u32 = 0;

    for weather_config in weather_configs {
      if !self.verify_weather(options, weather_config)? {
        invalid_weather_files_count += 1;
      }
    }

    let duration: u128 = started_at.elapsed().as_millis();
    let result = GamedataWeathersVerificationResult {
      duration,
      checked_weather_files_count,
      invalid_weather_files_count,
    };

    if options.is_logging_enabled() {
      println!("  - {}: {}", result.status(), result.failure_message());
    }

    Ok(result)
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

#[cfg(test)]
mod tests {
  use super::GamedataProject;
  use crate::{GamedataCheckResult, GamedataProjectVerifyOptions, GamedataVerificationStatus};
  use std::collections::HashMap;
  use std::fs;
  use std::path::{Path, PathBuf};
  use std::time::{SystemTime, UNIX_EPOCH};
  use xray_ltx::LtxProject;

  fn project_with_weather_files(root: &Path, ltx_files: Vec<PathBuf>) -> GamedataProject {
    GamedataProject {
      assets: HashMap::new(),
      ltx_project: LtxProject {
        root: root.to_path_buf(),
        ltx_file_entries: ltx_files.clone(),
        ltx_files,
        ltx_scheme_files: Vec::new(),
        ltx_scheme_file_entries: Vec::new(),
        ltx_scheme_declarations: Default::default(),
      },
      root: root.to_path_buf(),
    }
  }

  #[test]
  fn weather_parse_failure_makes_the_check_fail() {
    let unique = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Expected system time after Unix epoch")
      .as_nanos();
    let root = std::env::temp_dir().join(format!(
      "xray-gamedata-weather-test-{}-{unique}",
      std::process::id()
    ));
    let weathers = root.join("environment").join("weathers");
    fs::create_dir_all(&weathers).unwrap();

    let valid = weathers.join("valid.ltx");
    let invalid = weathers.join("invalid.ltx");
    fs::write(&valid, "[weather]\nvalue = valid\n").unwrap();
    fs::write(
      &invalid,
      "[weather]\nvalue = first\n[weather]\nvalue = duplicate\n",
    )
    .unwrap();

    let project = project_with_weather_files(&root, vec![valid, invalid]);
    let result = project
      .verify_weathers(&GamedataProjectVerifyOptions {
        is_silent: true,
        ..Default::default()
      })
      .unwrap();

    fs::remove_dir_all(&root).unwrap();

    assert_eq!(result.checked_weather_files_count, 2);
    assert_eq!(result.invalid_weather_files_count, 1);
    assert_eq!(result.status(), GamedataVerificationStatus::Failed);
    assert_eq!(
      result.failure_message(),
      "1/2 weather files failed to parse"
    );
  }
}

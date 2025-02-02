use crate::asset::asset_type::AssetType;
use crate::project::scripts::verify_scripts_result::GamedataScriptsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use full_moon::parse;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;
use xray_error::{XRayError, XRayResult};
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
    let checked_scripts_count: Mutex<u32> = Mutex::new(0);
    let invalid_scripts_count: Mutex<u32> = Mutex::new(0);

    self
      .get_all_asset_paths_by_type(AssetType::Script)
      .par_iter()
      .for_each(|path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify script: {}", path);
        }

        *checked_scripts_count.lock().unwrap() += 1;

        if let Some(path) = self.get_absolute_asset_path(path) {
          match self.verify_script(options, &path) {
            Ok(is_valid) => {
              if !is_valid {
                if options.is_logging_enabled() {
                  println!("Script is not valid: {}", path.display());
                }

                *invalid_scripts_count.lock().unwrap() += 1;
              }
            }
            Err(_) => {
              if options.is_logging_enabled() {
                println!("Script verification failed: {}", path.display());
              }

              *invalid_scripts_count.lock().unwrap() += 1;
            }
          }
        } else {
          if options.is_logging_enabled() {
            println!("Script path not found: {}", path);
          }

          *invalid_scripts_count.lock().unwrap() += 1;
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_scripts_count: u32 = *checked_scripts_count.lock().unwrap();
    let invalid_scripts_count: u32 = *invalid_scripts_count.lock().unwrap();

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
      invalid_scripts_count,
    })
  }

  pub fn verify_script(
    &self,
    _options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> XRayResult<bool> {
    let code: String = read_as_string_from_w1251_encoded(&mut File::open(path)?)?;

    parse(&code).map_err(|it| {
      XRayError::new_verify_error(format!(
        "Failed to check lua script file: {}, errors: {}",
        path.display(),
        it.iter()
          .map(|it| it.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ))
    })?;

    Ok(true)
  }
}

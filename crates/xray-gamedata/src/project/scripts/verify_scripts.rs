use crate::asset::asset_type::AssetType;
use crate::project::scripts::verify_scripts_result::GamedataScriptsVerificationResult;
use crate::{GamedataError, GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use encoding_rs::WINDOWS_1251;
use full_moon::parse;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

impl GamedataProject {
  pub fn verify_scripts(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataScriptsVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata scripts:".green(),);
    }

    let started_at: Instant = Instant::now();
    let checked_scripts_count: Mutex<u32> = Mutex::new(0);
    let invalid_scripts_count: Mutex<u32> = Mutex::new(0);

    self
      .get_all_asset_paths_by_type(AssetType::Script)
      .par_iter()
      .for_each(|path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify gamedata script: {}", path);
        }

        *checked_scripts_count.lock().unwrap() += 1;

        if let Some(path) = self.get_absolute_asset_path(path) {
          match self.verify_script(options, &path) {
            Ok(is_valid) => {
              if !is_valid {
                if options.is_logging_enabled() {
                  println!("Script is not valid: {:?}", path);
                }

                *invalid_scripts_count.lock().unwrap() += 1;
              }
            }
            Err(_) => {
              if options.is_logging_enabled() {
                println!("Script verification failed: {:?}", path);
              }

              *invalid_scripts_count.lock().unwrap() += 1;
            }
          }
        } else {
          if options.is_logging_enabled() {
            println!("Script path not found: {:?}", path);
          }

          *invalid_scripts_count.lock().unwrap() += 1;
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_scripts_count: u32 = *checked_scripts_count.lock().unwrap();
    let invalid_scripts_count: u32 = *invalid_scripts_count.lock().unwrap();

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata scripts in {} sec, {}/{} valid",
        (duration as f64) / 1000.0,
        checked_scripts_count - invalid_scripts_count,
        checked_scripts_count
      );
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
  ) -> GamedataResult<bool> {
    let code: String = Self::read_script_code(path)?;

    parse(&code).map_err(|it| {
      GamedataError::new_check_error(format!(
        "Failed to check lua script file: {:?}, errors: {}",
        path,
        it.iter()
          .map(|it| it.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ))
    })?;

    Ok(true)
  }
}

impl GamedataProject {
  pub fn read_script_code(path: &Path) -> GamedataResult<String> {
    let mut raw_data: Vec<u8> = Vec::new();
    let raw_data_read: usize = File::open(path)?.read_to_end(&mut raw_data)?;

    assert_eq!(
      raw_data_read,
      raw_data.len(),
      "Expected raw data size to match in-memory buffer"
    );

    let (cow, encoding_used, had_errors) = WINDOWS_1251.decode(&raw_data);

    if had_errors {
      Err(GamedataError::new_asset_error(format!(
        "Failed to read and decode script {:?} with {:?} encoding, {} bytes",
        path,
        encoding_used,
        raw_data.len()
      )))
    } else {
      Ok(cow.to_string())
    }
  }
}

use crate::asset::asset_type::AssetType;
use crate::project::scripts::verify_scripts_result::GamedataScriptsVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding};
use colored::Colorize;
use full_moon::{LuaVersion, parse_fallible};
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
    let findings: Mutex<Vec<GamedataVerificationFinding>> = Mutex::new(Vec::new());
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

                findings
                  .lock()
                  .unwrap()
                  .push(GamedataVerificationFinding::for_asset(
                    &path,
                    "LuaJIT parser rejected the script",
                  ));
                *invalid_scripts_count.lock().unwrap() += 1;
              }
            }
            Err(error) => {
              if options.is_logging_enabled() {
                eprintln!("Script verification failed: {error}");
              }

              findings
                .lock()
                .unwrap()
                .push(GamedataVerificationFinding::for_asset(
                  &path,
                  error.to_string(),
                ));
              *invalid_scripts_count.lock().unwrap() += 1;
            }
          }
        } else {
          if options.is_logging_enabled() {
            println!("Script path not found: {}", path);
          }

          findings
            .lock()
            .unwrap()
            .push(GamedataVerificationFinding::for_asset(
              Path::new(path),
              "Script path was not found in gamedata roots",
            ));
          *invalid_scripts_count.lock().unwrap() += 1;
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_scripts_count: u32 = *checked_scripts_count.lock().unwrap();
    let invalid_scripts_count: u32 = *invalid_scripts_count.lock().unwrap();
    let mut findings: Vec<GamedataVerificationFinding> =
      std::mem::take(&mut *findings.lock().unwrap());

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

fn verify_luajit_script(code: &str, path: &Path) -> XRayResult<()> {
  parse_fallible(code, LuaVersion::luajit())
    .into_result()
    .map(|_| ())
    .map_err(|errors| {
      XRayError::new_verify_error(format!(
        "Failed to check LuaJIT script file: {}, errors: {}",
        path.display(),
        errors
          .iter()
          .map(|it| it.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ))
    })
}

#[cfg(test)]
mod tests {
  use super::verify_luajit_script;
  use std::path::Path;

  #[test]
  fn accepts_luajit_goto_and_label() {
    let code: &str = r#"
local ____exports = {}

for index = 1, 3 do
  if index == 2 then
    goto __continue1
  end

  ::__continue1::
end

return ____exports
"#;

    assert!(verify_luajit_script(code, Path::new("generated.script")).is_ok());
  }

  #[test]
  fn rejects_invalid_luajit_syntax_with_path() {
    let path: &Path = Path::new("invalid.script");
    let error: String = verify_luajit_script("local value =", path)
      .expect_err("Expected malformed LuaJIT script to fail")
      .to_string();

    assert!(error.contains("invalid.script"));
    assert!(error.contains("errors:"));
  }
}

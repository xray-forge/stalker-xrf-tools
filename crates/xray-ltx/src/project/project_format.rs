use crate::project::project_format_result::LtxProjectFormatResult;
use crate::{Ltx, LtxFormatOptions, LtxProject, LtxResult};
use std::time::Instant;

impl LtxProject {
  /// Format all LTX entries in current project.
  pub fn format_all_files_opt(
    &self,
    options: LtxFormatOptions,
  ) -> LtxResult<LtxProjectFormatResult> {
    let mut result: LtxProjectFormatResult = LtxProjectFormatResult::new();
    let started_at: Instant = Instant::now();

    if !options.is_silent {
      println!("Formatting path: {:?}", self.root);
    }

    for entry in &self.ltx_files {
      if Ltx::format_file(entry.path(), true)? {
        result.invalid_files += 1;
        result.to_format.push(entry.path().into());

        if !options.is_silent {
          println!("Formatted: {:?}", entry.path());
        }
      } else {
        result.valid_files += 1;
      }

      result.total_files += 1;
    }

    result.duration = started_at.elapsed().as_millis();

    if !options.is_silent {
      println!(
        "Formatted {}/{} files in {} sec",
        result.invalid_files,
        self.ltx_file_entries.len(),
        (result.duration as f64) / 1000.0
      );
    }

    Ok(result)
  }

  /// Check format of all LTX entries in current project.
  pub fn check_format_all_files_opt(
    &self,
    options: LtxFormatOptions,
  ) -> LtxResult<LtxProjectFormatResult> {
    let mut result: LtxProjectFormatResult = LtxProjectFormatResult::new();
    let started_at: Instant = Instant::now();

    if options.is_logging_enabled() {
      println!("Checking path: {:?}", self.root);
    }

    for entry in &self.ltx_files {
      if Ltx::format_file(entry.path(), false)? {
        result.invalid_files += 1;
        result.to_format.push(entry.path().into());

        if options.is_logging_enabled() {
          println!("Not formatted: {:?}", entry.path());
        }
      } else {
        result.valid_files += 1;
      }

      result.total_files += 1;
    }

    result.duration = started_at.elapsed().as_millis();

    if options.is_logging_enabled() {
      if result.invalid_files == 0 {
        println!(
          "All {} files are formatted, checked in {} sec",
          self.ltx_file_entries.len(),
          (result.duration as f64) / 1000.0
        );
      } else {
        println!(
          "Format issues with {}/{} files in {} sec",
          result.invalid_files,
          self.ltx_file_entries.len(),
          (result.duration as f64) / 1000.0
        );
      }
    }

    Ok(result)
  }

  /// Format all LTX entries in current project.
  pub fn format_all_files(&self) -> LtxResult<LtxProjectFormatResult> {
    self.format_all_files_opt(LtxFormatOptions::default())
  }

  /// Format all LTX entries in current project.
  pub fn check_format_all_files(&self) -> LtxResult<LtxProjectFormatResult> {
    self.check_format_all_files_opt(LtxFormatOptions::default())
  }
}

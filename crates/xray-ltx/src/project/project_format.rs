use crate::project::project_format_result::LtxProjectFormatResult;
use crate::{Ltx, LtxError, LtxFormatOptions, LtxProject};

impl LtxProject {
  /// Format all LTX entries in current project.
  pub fn format_all_files_opt(
    &self,
    options: LtxFormatOptions,
  ) -> Result<LtxProjectFormatResult, LtxError> {
    let mut result: LtxProjectFormatResult = LtxProjectFormatResult::new();

    if !options.is_silent {
      println!("Formatting path: {:?}", self.root);
    }

    for entry in &self.ltx_files {
      if Ltx::format_file(entry.path(), true)? {
        result.invalid.push(entry.path().into());

        if !options.is_silent {
          println!("Formatted: {:?}", entry.path());
        }
      } else {
        result.valid.push(entry.path().into());
      }

      result.total += 1;
    }

    if !options.is_silent {
      println!(
        "Formatted {}/{} files",
        result.invalid.len(),
        self.ltx_file_entries.len()
      );
    }

    Ok(result)
  }

  /// Check format of all LTX entries in current project.
  pub fn check_format_all_files_opt(
    &self,
    options: LtxFormatOptions,
  ) -> Result<LtxProjectFormatResult, LtxError> {
    let mut result: LtxProjectFormatResult = LtxProjectFormatResult::new();

    if !options.is_silent {
      println!("Checking path: {:?}", self.root);
    }

    for entry in &self.ltx_files {
      if Ltx::format_file(entry.path(), false)? {
        result.invalid.push(entry.path().into());

        if !options.is_silent {
          println!("Not formatted: {:?}", entry.path());
        }
      } else {
        result.valid.push(entry.path().into());
      }

      result.total += 1;
    }

    if !options.is_silent {
      if result.invalid.is_empty() {
        println!("All {} files are formatted", self.ltx_file_entries.len());
      } else {
        println!(
          "Format issues with {}/{} files",
          result.invalid.len(),
          self.ltx_file_entries.len()
        );
      }
    }

    Ok(result)
  }

  /// Format all LTX entries in current project.
  pub fn format_all_files(&self) -> Result<LtxProjectFormatResult, LtxError> {
    self.format_all_files_opt(LtxFormatOptions::default())
  }

  /// Format all LTX entries in current project.
  pub fn check_format_all_files(&self) -> Result<LtxProjectFormatResult, LtxError> {
    self.check_format_all_files_opt(LtxFormatOptions::default())
  }
}

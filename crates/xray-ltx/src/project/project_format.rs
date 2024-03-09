use crate::{Ltx, LtxError, LtxFormatOptions, LtxProject};

impl LtxProject {
  /// Format all LTX entries in current project.
  pub fn format_all_files_opt(&self, options: LtxFormatOptions) -> Result<bool, LtxError> {
    let mut count: usize = 0;

    if !options.is_silent {
      println!("Formatting path: {:?}", self.root);
    }

    for entry in &self.ltx_files {
      if Ltx::format_file(entry.path(), true)? {
        count += 1;

        if !options.is_silent {
          println!("Formatted: {:?}", entry.path());
        }
      }
    }

    if !options.is_silent {
      println!("Formatted {count}/{} files", self.ltx_file_entries.len());
    }

    Ok(count > 0)
  }

  /// Check format of all LTX entries in current project.
  pub fn check_format_all_files_opt(&self, options: LtxFormatOptions) -> Result<bool, LtxError> {
    let mut count: usize = 0;

    if !options.is_silent {
      println!("Checking path: {:?}", self.root);
    }

    for entry in &self.ltx_files {
      if Ltx::format_file(entry.path(), false)? {
        count += 1;

        if !options.is_silent {
          println!("Not formatted: {:?}", entry.path());
        }
      }
    }

    if !options.is_silent {
      if count > 0 {
        println!(
          "Format issues with {count}/{} files",
          self.ltx_file_entries.len()
        );
      } else {
        println!("All {} files are formatted", self.ltx_file_entries.len());
      }
    }

    Ok(count > 0)
  }

  /// Format all LTX entries in current project.
  pub fn format_all_files(&self) -> Result<bool, LtxError> {
    self.format_all_files_opt(LtxFormatOptions::default())
  }

  /// Format all LTX entries in current project.
  pub fn check_format_all_files(&self) -> Result<bool, LtxError> {
    self.check_format_all_files_opt(LtxFormatOptions::default())
  }
}

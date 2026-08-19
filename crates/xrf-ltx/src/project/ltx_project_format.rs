use std::path::PathBuf;

use xrf_error::{XrfError, XrfResult};

use crate::project::ltx_files_formatter::LtxFilesFormatter;
use crate::project::ltx_project_format_result::LtxProjectFormatResult;
use crate::{LtxFormatOptions, LtxProject};

impl LtxProject {
  /// Format all LTX entries in current project.
  pub fn format_all_files_opt(&self, options: LtxFormatOptions) -> XrfResult<LtxProjectFormatResult> {
    LtxFilesFormatter::format_opt(&self.writable_files()?, options)
  }

  /// Check format of all LTX entries in current project.
  pub fn check_format_all_files_opt(&self, options: LtxFormatOptions) -> XrfResult<LtxProjectFormatResult> {
    LtxFilesFormatter::check_format_opt(&self.writable_files()?, options)
  }

  /// The filesystem paths of this project's files, refusing when any of them has none.
  ///
  /// Formatting rewrites a file in place, which an archived config cannot do. Refusing by name is deliberate: a project
  /// spanning an installation would otherwise format the loose handful and report success over thousands it never touched.
  /// A caller that wants the loose subset should select it explicitly, as `xrf-cli format-ltx` does.
  fn writable_files(&self) -> XrfResult<Vec<PathBuf>> {
    let mut writable: Vec<PathBuf> = Vec::with_capacity(self.ltx_files.len());

    for logical_path in &self.ltx_files {
      match self.physical_path_of(logical_path) {
        Some(physical) => writable.push(physical),
        None => {
          return Err(XrfError::new_asset_error(format!(
            "Cannot format '{}': it has no file on disk, being read out of an archive",
            logical_path.display()
          )));
        }
      }
    }

    Ok(writable)
  }

  /// Format all LTX entries in current project.
  pub fn format_all_files(&self) -> XrfResult<LtxProjectFormatResult> {
    self.format_all_files_opt(LtxFormatOptions::default())
  }

  /// Format all LTX entries in current project.
  pub fn check_format_all_files(&self) -> XrfResult<LtxProjectFormatResult> {
    self.check_format_all_files_opt(LtxFormatOptions::default())
  }
}

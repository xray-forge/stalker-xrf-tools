use std::path::PathBuf;

use xrf_error::{XrfError, XrfResult};

use crate::project::ltx_files_formatter::LtxFilesFormatter;
use crate::project::ltx_project_format_result::LtxProjectFormatResult;
use crate::{LtxFormatOptions, LtxProject};

impl LtxProject {
  /// Formats every project LTX file with explicit options.
  ///
  /// Returns an error instead of partially formatting a project when any config is archived.
  pub fn format_all_files_opt(&self, options: LtxFormatOptions) -> XrfResult<LtxProjectFormatResult> {
    LtxFilesFormatter::format_opt(&self.writable_files()?, options)
  }

  /// Checks every project LTX file with explicit options.
  ///
  /// Returns an error when any config is archived, matching [`Self::format_all_files_opt`].
  pub fn check_format_all_files_opt(&self, options: LtxFormatOptions) -> XrfResult<LtxProjectFormatResult> {
    LtxFilesFormatter::check_format_opt(&self.writable_files()?, options)
  }

  /// Returns physical paths for every project file, refusing when one is not loose.
  ///
  /// Formatting rewrites a file in place, which an archived config cannot do. Refusing by name is deliberate: a project
  /// spanning an installation would otherwise format the loose handful and report success over thousands it never touched.
  /// A caller that needs the loose subset must select it explicitly.
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

  /// Formats every project LTX file with default options.
  pub fn format_all_files(&self) -> XrfResult<LtxProjectFormatResult> {
    self.format_all_files_opt(LtxFormatOptions::default())
  }

  /// Checks every project LTX file with default options.
  pub fn check_format_all_files(&self) -> XrfResult<LtxProjectFormatResult> {
    self.check_format_all_files_opt(LtxFormatOptions::default())
  }
}

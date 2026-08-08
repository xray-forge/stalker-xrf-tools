use crate::project::ltx_files_formatter::LtxFilesFormatter;
use crate::project::ltx_project_format_result::LtxProjectFormatResult;
use crate::{LtxFormatOptions, LtxProject};
use xray_error::XRayResult;

impl LtxProject {
  /// Format all LTX entries in current project.
  pub fn format_all_files_opt(
    &self,
    options: LtxFormatOptions,
  ) -> XRayResult<LtxProjectFormatResult> {
    LtxFilesFormatter::format_opt(&self.ltx_files, options)
  }

  /// Check format of all LTX entries in current project.
  pub fn check_format_all_files_opt(
    &self,
    options: LtxFormatOptions,
  ) -> XRayResult<LtxProjectFormatResult> {
    LtxFilesFormatter::check_format_opt(&self.ltx_files, options)
  }

  /// Format all LTX entries in current project.
  pub fn format_all_files(&self) -> XRayResult<LtxProjectFormatResult> {
    self.format_all_files_opt(LtxFormatOptions::default())
  }

  /// Format all LTX entries in current project.
  pub fn check_format_all_files(&self) -> XRayResult<LtxProjectFormatResult> {
    self.check_format_all_files_opt(LtxFormatOptions::default())
  }
}

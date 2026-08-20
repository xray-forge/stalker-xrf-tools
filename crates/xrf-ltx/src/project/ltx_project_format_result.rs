use std::path::PathBuf;

use serde::Serialize;

use crate::LtxFormatOptions;

#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LtxProjectFormatResult {
  pub duration: u128,
  pub invalid_files: usize,
  pub to_format: Vec<PathBuf>,
  pub total_files: usize,
  pub valid_files: usize,
}

impl LtxProjectFormatResult {
  pub fn new() -> Self {
    Self {
      duration: 0,
      invalid_files: 0,
      to_format: Vec::new(),
      total_files: 0,
      valid_files: 0,
    }
  }

  /// Records the verdict for one checked config and reports it.
  ///
  /// Shared by the file-based formatter and the project check, which differ only in how they obtain the contents.
  pub(crate) fn record_checked(&mut self, path: PathBuf, is_formatted: bool, options: &LtxFormatOptions) {
    if is_formatted {
      self.valid_files += 1;
    } else {
      xrf_output::info!(options.output, "Not formatted: {}", path.display());

      self.invalid_files += 1;
      self.to_format.push(path);
    }

    self.total_files += 1;
  }
}

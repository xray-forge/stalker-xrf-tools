use std::path::PathBuf;
use std::time::Duration;

use serde::Serialize;

/// What one packing run produced.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePackResult {
  /// Volumes written, in mount order.
  pub volumes: Vec<PathBuf>,
  pub files_total: usize,
  /// Files the include, exclude, and skip rules left out.
  pub files_skipped: usize,
  pub files_stored: usize,
  pub files_compressed: usize,
  /// Files that shared an identical earlier payload and cost only a descriptor row.
  pub files_aliased: usize,
  pub size_source: u64,
  pub size_written: u64,
  #[serde(with = "xrf_utils::duration_ms")]
  #[cfg_attr(feature = "typescript-bindings", specta(type = u64))]
  pub duration: Duration,
}

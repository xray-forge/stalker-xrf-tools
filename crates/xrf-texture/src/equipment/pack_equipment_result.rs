use std::path::PathBuf;
use std::time::Duration;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct PackEquipmentResult {
  #[serde(with = "xrf_utils::duration_ms")]
  #[cfg_attr(feature = "typescript-bindings", specta(type = u64))]
  pub duration: Duration,
  pub saved_at: PathBuf,
  pub saved_width: u32,
  pub saved_height: u32,
  pub packed_count: u32,
  pub skipped_count: u32,
}

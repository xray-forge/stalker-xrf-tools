use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackEquipmentResult {
  pub duration: u128,
  pub saved_at: PathBuf,
  pub saved_width: u32,
  pub saved_height: u32,
  pub packed_count: u32,
  pub skipped_count: u32,
}

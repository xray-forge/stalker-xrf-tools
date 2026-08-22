use xrf_pack::ArchivePackConfig;

use crate::core::error::error_to_string;
use crate::core::types::TauriResult;

/// Write the selection rules of a configuration out as an xrCompress configuration file.
///
/// Only what such a file can carry is written, so a round trip through import returns what was exported.
/// Paths, name, mode, and volume size belong to the run rather than to the file.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "export_pack_config"))]
#[tauri::command(rename = "export_pack_config")]
pub async fn archives_export_pack_config(path: &str, config: ArchivePackConfig) -> TauriResult<()> {
  log::info!("Exporting pack config: {path}");

  config.write_ltx_to_path(path).map_err(error_to_string)
}

use xrf_archive::ArchivePackConfig;

use crate::types::TauriResult;
use crate::utils::error_to_string;

/// Read an xrCompress configuration file over the configuration the caller holds.
///
/// Layers rather than replaces, matching how the command line applies `--ltx`: a configuration file
/// carries selection rules and a header, never the source, destination, name, mode, or volume size, so
/// those stay as the caller had them.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "import_pack_config"))]
#[tauri::command(rename = "import_pack_config")]
pub async fn archives_import_pack_config(path: &str, config: ArchivePackConfig) -> TauriResult<ArchivePackConfig> {
  log::info!("Importing pack config: {path}");

  config.with_ltx_file(path).map_err(error_to_string)
}

use xrf_archive::{ArchivePackConfig, ArchivePackResult, ArchivePacker};

use crate::app::types::TauriResult;
use crate::app::utils::error_to_string;

/// Pack a directory into archive volumes from a configuration held by the caller.
///
/// Takes the whole configuration rather than a file path, so the editor packs exactly what is on screen
/// without having to save it first.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "pack_directory"))]
#[tauri::command(rename = "pack_directory")]
pub async fn archives_pack_directory(config: ArchivePackConfig) -> TauriResult<ArchivePackResult> {
  log::info!(
    "Packing archive: {} -> {} as '{}'",
    config.source.display(),
    config.destination.display(),
    config.name
  );

  ArchivePacker::pack(&config).map_err(error_to_string)
}

use xrf_archive::{ArchivePackConfig, ArchivePackMode, ArchivePackResult, ArchivePacker};

use crate::types::TauriResult;
use crate::utils::error_to_string;

/// Bytes in the megabytes the volume size is given in, matching the command line.
const BYTES_PER_MEGABYTE: u64 = 1024 * 1024;

/// Pack a directory into archive volumes.
///
/// Layers the same way the command line does: defaults, then an optional configuration file, then the
/// values the caller supplied, so a form and a command line produce the same archive from the same input.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "pack_directory"))]
#[tauri::command(rename = "pack_directory")]
pub async fn archives_pack_directory(
  source_path: &str,
  destination_path: &str,
  name: &str,
  ltx_path: Option<&str>,
  is_store: bool,
  max_size_megabytes: Option<u32>,
) -> TauriResult<ArchivePackResult> {
  log::info!("Packing archive: {source_path} -> {destination_path} as '{name}'");

  let mut config: ArchivePackConfig = ArchivePackConfig::new(source_path, destination_path, name);

  if let Some(ltx_path) = ltx_path {
    log::info!("Packing with config: {ltx_path}");

    config = config.with_ltx_file(ltx_path).map_err(error_to_string)?;
  }

  if is_store {
    config.mode = ArchivePackMode::Store;
  }

  if let Some(megabytes) = max_size_megabytes {
    config = config
      .with_max_volume_size(u64::from(megabytes) * BYTES_PER_MEGABYTE)
      .map_err(error_to_string)?;
  }

  ArchivePacker::pack(&config).map_err(error_to_string)
}

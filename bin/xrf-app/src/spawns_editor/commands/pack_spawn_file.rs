use std::path::Path;

use xrf_db::{SpawnFile, XRayByteOrder};

use crate::types::TauriResult;
use crate::utils::error_to_string;

/// Build a packed spawn file from unpacked chunks on disk.
#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn pack_spawn_file(from: &str, destination: &str) -> TauriResult {
  log::info!("Packing spawn file from: {}", from);

  let file: SpawnFile = SpawnFile::import_from_path::<XRayByteOrder, _>(&Path::new(from)).map_err(error_to_string)?;

  log::info!("Packing spawn file into: {}", destination);

  file
    .write_to_path::<XRayByteOrder, _>(&Path::new(destination))
    .map_err(error_to_string)
}

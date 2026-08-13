use std::path::Path;

use xrf_db::{SpawnFile, XRayByteOrder};

use crate::types::TauriResult;
use crate::utils::error_to_string;

/// Expand a packed spawn file into editable chunks on disk.
#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn unpack_spawn_file(from: &str, destination: &str) -> TauriResult {
  log::info!("Unpacking spawn file: {}", from);

  let file: SpawnFile = SpawnFile::read_from_path::<XRayByteOrder, _>(&Path::new(from)).map_err(error_to_string)?;

  log::info!("Unpacking spawn file into: {}", destination);

  file
    .export_to_path::<XRayByteOrder, _>(&Path::new(destination))
    .map_err(error_to_string)
}

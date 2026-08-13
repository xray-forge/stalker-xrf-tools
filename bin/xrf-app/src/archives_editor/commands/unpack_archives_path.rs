use std::path::Path;

use xrf_archive::{ArchiveProject, ArchiveUnpackResult};

use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn unpack_archives_path(from: &str, destination: &str) -> TauriResult<ArchiveUnpackResult> {
  log::info!("Open archive folder: {}", from);

  let project: ArchiveProject = ArchiveProject::new(&Path::new(from)).map_err(error_to_string)?;

  log::info!("Unpacking archive to: {}", destination);

  match project.unpack_parallel(destination, 32).await {
    Ok(result) => Ok(result),
    Err(error) => Err(error.to_string()),
  }
}

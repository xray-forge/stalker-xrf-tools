use std::path::Path;

use xrf_archive::{ArchiveProject, ArchiveUnpackResult};

use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "unpack_directory"))]
#[tauri::command(rename = "unpack_directory")]
pub async fn archives_unpack_directory(from: &str, destination: &str) -> TauriResult<ArchiveUnpackResult> {
  log::info!("Open archive directory: {}", from);

  let project: ArchiveProject = ArchiveProject::new(&Path::new(from)).map_err(error_to_string)?;

  log::info!("Unpacking archive to: {}", destination);

  match project.unpack_parallel(destination, 32).await {
    Ok(result) => Ok(result),
    Err(error) => Err(error.to_string()),
  }
}

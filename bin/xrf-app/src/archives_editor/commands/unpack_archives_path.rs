use crate::types::TauriResult;
use crate::utils::error_to_string;
use serde_json::{json, Value};
use std::path::Path;
use xray_archive::ArchiveProject;

#[tauri::command]
pub async fn unpack_archives_path(from: &str, destination: &str) -> TauriResult<Value> {
  log::info!("Open archive folder: {}", from);

  let project: ArchiveProject = ArchiveProject::new(&Path::new(from)).map_err(error_to_string)?;

  log::info!("Unpacking archive to: {}", destination);

  match project.unpack_parallel(destination, 32).await {
    Ok(result) => Ok(json!(result)),
    Err(error) => Err(error.to_string()),
  }
}

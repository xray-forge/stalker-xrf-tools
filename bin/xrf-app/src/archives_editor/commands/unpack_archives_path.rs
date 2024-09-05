use serde_json::{json, Value};
use std::path::PathBuf;
use xray_archive::ArchiveProject;

#[tauri::command]
pub async fn unpack_archives_path(from: &str, destination: &str) -> Result<Value, String> {
  log::info!("Open archive folder: {:?}", from);

  let project: ArchiveProject = match ArchiveProject::new(&PathBuf::from(from)) {
    Ok(project) => project,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Unpacking archive to: {:?}", destination);

  match project
    .unpack_parallel(&PathBuf::from(destination), 32)
    .await
  {
    Ok(result) => Ok(json!(result)),
    Err(error) => Err(error.to_string()),
  }
}

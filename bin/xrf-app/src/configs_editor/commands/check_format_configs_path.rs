use serde_json::{json, Value};
use std::path::PathBuf;
use xray_ltx::{LtxFormatOptions, LtxProject};

#[tauri::command]
pub async fn check_format_configs_path(path: &str) -> Result<Value, String> {
  log::info!("Open ltx folder: {:?}", path);

  let project: LtxProject = match LtxProject::open_at_path(&PathBuf::from(path)) {
    Ok(project) => project,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Check format for ltx folder: {:?}", path);

  match project.check_format_all_files_opt(LtxFormatOptions { is_silent: true }) {
    Ok(result) => Ok(json!(result)),
    Err(error) => Err(error.to_string()),
  }
}

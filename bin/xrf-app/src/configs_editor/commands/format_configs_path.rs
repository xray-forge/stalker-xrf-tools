use serde_json::{Value, json};
use xray_ltx::{LtxFormatOptions, LtxProject, LtxProjectFormatResult};

use crate::types::TauriResult;
use crate::utils::error_to_string;

#[tauri::command]
pub async fn format_configs_path(path: &str) -> TauriResult<Value> {
  log::info!("Open ltx folder: {}", path);

  let project: LtxProject = LtxProject::open_at_path(path).map_err(error_to_string)?;

  log::info!("Formatting ltx folder: {}", path);

  let result: LtxProjectFormatResult = project
    .format_all_files_opt(LtxFormatOptions::default())
    .map_err(error_to_string)?;

  Ok(json!(result))
}

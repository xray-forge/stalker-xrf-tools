use crate::types::TauriResult;
use crate::utils::error_to_string;
use serde_json::{json, Value};
use xray_ltx::{LtxFormatOptions, LtxProject, LtxProjectFormatResult};

#[tauri::command]
pub async fn check_format_configs_path(path: &str) -> TauriResult<Value> {
  log::info!("Open ltx folder: {}", path);

  let project: LtxProject = LtxProject::open_at_path(path).map_err(error_to_string)?;

  log::info!("Check format for ltx folder: {}", path);

  let result: LtxProjectFormatResult = project
    .check_format_all_files_opt(LtxFormatOptions {
      is_silent: true,
      is_verbose: false,
    })
    .map_err(error_to_string)?;

  Ok(json!(result))
}

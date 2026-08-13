use xrf_ltx::{LtxFormatOptions, LtxProject, LtxProjectFormatResult};

use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn check_format_configs_path(path: &str) -> TauriResult<LtxProjectFormatResult> {
  log::info!("Open ltx folder: {}", path);

  let project: LtxProject = LtxProject::open_at_path(path).map_err(error_to_string)?;

  log::info!("Check format for ltx folder: {}", path);

  let result: LtxProjectFormatResult = project
    .check_format_all_files_opt(LtxFormatOptions::default())
    .map_err(error_to_string)?;

  Ok(result)
}

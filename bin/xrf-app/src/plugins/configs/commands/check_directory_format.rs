use xrf_ltx::{LtxFormatOptions, LtxProject, LtxProjectFormatResult};

use crate::app::types::TauriResult;
use crate::app::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "check_directory_format"))]
#[tauri::command(rename = "check_directory_format")]
pub async fn configs_check_directory_format(path: &str) -> TauriResult<LtxProjectFormatResult> {
  log::info!("Open ltx directory: {}", path);

  let project: LtxProject = LtxProject::open_at_path(path).map_err(error_to_string)?;

  log::info!("Check format for ltx directory: {}", path);

  let result: LtxProjectFormatResult = project
    .check_format_all_files_opt(LtxFormatOptions::default())
    .map_err(error_to_string)?;

  Ok(result)
}

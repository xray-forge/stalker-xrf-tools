use xrf_ltx::{LtxFormatOptions, LtxProject, LtxProjectFormatResult};

use crate::core::error::error_to_string;
use crate::core::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "format_directory"))]
#[tauri::command(rename = "format_directory")]
pub async fn configs_format_directory(path: &str) -> TauriResult<LtxProjectFormatResult> {
  log::info!("Open ltx directory: {}", path);

  let project: LtxProject = LtxProject::open_at_path(path).map_err(error_to_string)?;

  log::info!("Formatting ltx directory: {}", path);

  let result: LtxProjectFormatResult = project
    .format_all_files_opt(LtxFormatOptions::default())
    .map_err(error_to_string)?;

  Ok(result)
}

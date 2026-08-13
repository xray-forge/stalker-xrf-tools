use xrf_ltx::{LtxProject, LtxProjectOptions, LtxProjectVerifyResult, LtxVerifyOptions};

use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "verify_directory"))]
#[tauri::command(rename = "verify_directory")]
pub async fn configs_verify_directory(path: &str) -> TauriResult<LtxProjectVerifyResult> {
  log::info!("Open ltx directory: {}", path);

  let project: LtxProject = LtxProject::open_at_path_opt(
    path,
    LtxProjectOptions {
      is_with_schemes_check: true,
      // todo: Probably should be provided as parameter.
      is_strict_check: false,
    },
  )
  .map_err(error_to_string)?;

  log::info!("Verifying ltx directory: {}", path);

  let result: LtxProjectVerifyResult = project
    .verify_entries_opt(LtxVerifyOptions::default())
    .map_err(error_to_string)?;

  Ok(result)
}

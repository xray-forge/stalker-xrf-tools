use crate::types::TauriResult;
use crate::utils::error_to_string;
use serde_json::{json, Value};
use std::path::PathBuf;
use xray_ltx::{LtxProject, LtxProjectOptions, LtxProjectVerifyResult, LtxVerifyOptions};

#[tauri::command]
pub async fn verify_configs_path(path: &str) -> TauriResult<Value> {
  log::info!("Open ltx folder: {}", path);

  let project: LtxProject = LtxProject::open_at_path_opt(
    &PathBuf::from(path),
    LtxProjectOptions {
      is_with_schemes_check: true,
      // todo: Probably should be provided as parameter.
      is_strict_check: false,
    },
  )
  .map_err(error_to_string)?;

  log::info!("Verifying ltx folder: {}", path);

  let result: LtxProjectVerifyResult = project
    .verify_entries_opt(LtxVerifyOptions {
      is_silent: true,
      is_verbose: false,
      is_strict: false,
    })
    .map_err(error_to_string)?;

  Ok(json!(result))
}

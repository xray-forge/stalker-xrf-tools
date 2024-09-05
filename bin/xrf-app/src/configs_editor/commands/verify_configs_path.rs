use serde_json::{json, Value};
use std::path::PathBuf;
use xray_ltx::{LtxProject, LtxProjectOptions, LtxVerifyOptions};

#[tauri::command]
pub async fn verify_configs_path(path: &str) -> Result<Value, String> {
  log::info!("Open ltx folder: {:?}", path);

  let project: LtxProject = match LtxProject::open_at_path_opt(
    &PathBuf::from(path),
    LtxProjectOptions {
      is_with_schemes_check: true,
    },
  ) {
    Ok(project) => project,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Verifying ltx folder: {:?}", path);

  match project.verify_entries_opt(LtxVerifyOptions {
    is_silent: true,
    is_verbose: false,
    is_strict: false,
  }) {
    Ok(result) => Ok(json!(result)),
    Err(error) => Err(error.to_string()),
  }
}

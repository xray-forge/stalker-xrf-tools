use serde_json::{Value, json};
use xray_export::{ExportDescriptor, ExportsEditorParser};

use crate::types::TauriResult;
use crate::utils::error_to_string;

#[tauri::command]
pub async fn parse_xr_effects(path: &str) -> TauriResult<Value> {
  log::info!("Parsing effects exports folder: {}", path);

  let value: Vec<ExportDescriptor> = ExportsEditorParser::new()
    .parse_effects_from_path(path)
    .map_err(error_to_string)?;

  Ok(json!(value))
}

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;
use serde_json::{json, Value};
use tauri::State;
use xray_export::{ExportDescriptor, ExportsParser};

#[tauri::command]
pub async fn open_xr_effects(
  path: &str,
  state: State<'_, ExportsEditorState>,
) -> TauriResult<Value> {
  log::info!("Parsing effects exports folder: {}", path);

  let value: Vec<ExportDescriptor> = ExportsParser::new()
    .parse_effects_from_path(path)
    .map_err(error_to_string)?;

  let json: Value = json!(value);

  *state.effects.lock().unwrap() = Some(value);

  Ok(json)
}

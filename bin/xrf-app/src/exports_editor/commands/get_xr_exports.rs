use serde_json::{Value, json};
use tauri::State;
use xray_export::ExportDescriptor;

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub async fn get_xr_exports(state: State<'_, ExportsEditorState>) -> TauriResult<Option<Value>> {
  let declarations: Option<Vec<ExportDescriptor>> = state.exports.lock().unwrap().as_ref().cloned();

  Ok(declarations.map(|declarations: Vec<ExportDescriptor>| json!(declarations)))
}

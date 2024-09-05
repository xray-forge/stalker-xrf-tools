use crate::exports_editor::state::ExportsEditorState;
use serde_json::{json, Value};
use std::sync::MutexGuard;
use tauri::State;
use xray_export::ExportDescriptor;

#[tauri::command]
pub async fn get_xr_effects(state: State<'_, ExportsEditorState>) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.effects.lock().unwrap();

  if (*lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap())))
}

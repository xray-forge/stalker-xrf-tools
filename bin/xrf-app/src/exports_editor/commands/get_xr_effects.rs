use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;
use serde_json::{json, Value};
use std::sync::MutexGuard;
use tauri::State;
use xray_export::ExportDescriptor;

#[tauri::command]
pub async fn get_xr_effects(state: State<'_, ExportsEditorState>) -> TauriResult<Option<Value>> {
  let lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.effects.lock().unwrap();

  if (*lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap())))
}

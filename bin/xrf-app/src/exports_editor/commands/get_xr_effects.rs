use std::sync::MutexGuard;

use serde_json::{Value, json};
use tauri::State;
use xray_export::ExportDescriptor;

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub async fn get_xr_effects(state: State<'_, ExportsEditorState>) -> TauriResult<Option<Value>> {
  let lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.effects.lock().unwrap();

  if (*lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap())))
}

use tauri::State;

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub fn has_xr_effects(state: State<'_, ExportsEditorState>) -> TauriResult<bool> {
  Ok(state.effects.lock().unwrap().is_some())
}

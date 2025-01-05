use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;
use tauri::State;

#[tauri::command]
pub fn has_xr_effects(state: State<'_, ExportsEditorState>) -> TauriResult<bool> {
  Ok(state.effects.lock().unwrap().is_some())
}

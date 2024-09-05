use crate::exports_editor::state::ExportsEditorState;
use tauri::State;

#[tauri::command]
pub fn has_xr_effects(state: State<'_, ExportsEditorState>) -> bool {
  state.effects.lock().unwrap().is_some()
}

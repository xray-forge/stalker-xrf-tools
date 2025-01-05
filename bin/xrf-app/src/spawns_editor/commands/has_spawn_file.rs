use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;
use tauri::State;

#[tauri::command]
pub fn has_spawn_file(state: State<'_, SpawnsEditorState>) -> TauriResult<bool> {
  Ok(state.file.lock().unwrap().is_some())
}

use crate::spawns_editor::state::SpawnsEditorState;
use tauri::State;

#[tauri::command]
pub fn has_spawn_file(state: State<'_, SpawnsEditorState>) -> bool {
  state.file.lock().unwrap().is_some()
}

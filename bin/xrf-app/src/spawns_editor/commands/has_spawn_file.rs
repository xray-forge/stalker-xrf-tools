use tauri::State;

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub fn has_spawn_file(state: State<'_, SpawnsEditorState>) -> TauriResult<bool> {
  log::debug!("Checking spawn file presence");

  Ok(state.file.lock().unwrap().is_some())
}

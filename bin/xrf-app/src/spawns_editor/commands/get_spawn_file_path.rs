use std::sync::MutexGuard;

use tauri::State;

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

/// Where the open file came from, so a restored session can name what it is showing.
#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub fn get_spawn_file_path(state: State<'_, SpawnsEditorState>) -> TauriResult<Option<String>> {
  log::debug!("Getting spawn file path");

  let lock: MutexGuard<Option<String>> = state.path.lock().unwrap();

  Ok(lock.clone())
}

use std::sync::MutexGuard;

use tauri::State;
use xrf_db::SpawnFile;

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub fn close_spawn_file(state: State<'_, SpawnsEditorState>) -> TauriResult {
  log::info!("Closing spawn file");

  let mut lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  // Released with the file rather than left behind.
  *state.path.lock().unwrap() = None;

  Ok(())
}

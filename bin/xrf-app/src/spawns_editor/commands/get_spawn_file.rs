use std::sync::MutexGuard;

use tauri::State;
use xrf_db::SpawnFile;

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn get_spawn_file(state: State<'_, SpawnsEditorState>) -> TauriResult<Option<SpawnFile>> {
  log::debug!("Getting spawn file");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  Ok(lock.clone())
}

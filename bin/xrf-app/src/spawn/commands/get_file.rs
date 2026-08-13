use std::sync::MutexGuard;

use tauri::State;
use xrf_db::SpawnFile;

use crate::spawn::state::SpawnFileState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_file"))]
#[tauri::command(rename = "get_file")]
pub async fn spawn_get_file(state: State<'_, SpawnFileState>) -> TauriResult<Option<SpawnFile>> {
  log::debug!("Getting spawn file");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  Ok(lock.clone())
}

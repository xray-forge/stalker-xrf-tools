use std::sync::MutexGuard;

use tauri::State;
use xrf_db::SpawnFile;

use crate::spawn::state::SpawnFileState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "close_file"))]
#[tauri::command(rename = "close_file")]
pub fn spawn_close_file(state: State<'_, SpawnFileState>) -> TauriResult {
  log::info!("Closing spawn file");

  let mut lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  // Released with the file rather than left behind.
  *state.path.lock().unwrap() = None;

  Ok(())
}

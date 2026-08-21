use std::sync::MutexGuard;

use tauri::State;

use crate::core::types::TauriResult;
use crate::plugins::spawn::state::SpawnFileState;

/// Where the open file came from, so a restored session can name what it is showing.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_path"))]
#[tauri::command(rename = "get_path")]
pub fn spawn_get_path(state: State<'_, SpawnFileState>) -> TauriResult<Option<String>> {
  log::debug!("Getting spawn file path");

  let lock: MutexGuard<Option<String>> = state.path.lock().unwrap();

  Ok(lock.clone())
}

use std::sync::MutexGuard;

use tauri::State;
use xrf_db::{SpawnFile, SpawnPatrolsChunk};

use crate::app::types::TauriResult;
use crate::plugins::spawn::state::SpawnFileState;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_patrols"))]
#[tauri::command(rename = "get_patrols")]
pub async fn spawn_get_patrols(state: State<'_, SpawnFileState>) -> TauriResult<Option<SpawnPatrolsChunk>> {
  log::debug!("Getting spawn file patrols");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  Ok(lock.as_ref().map(|file| file.patrols.clone()))
}

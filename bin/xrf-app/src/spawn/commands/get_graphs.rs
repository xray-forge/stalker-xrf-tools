use std::sync::MutexGuard;

use tauri::State;
use xrf_db::{SpawnFile, SpawnGraphsChunk};

use crate::spawn::state::SpawnFileState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_graphs"))]
#[tauri::command(rename = "get_graphs")]
pub async fn spawn_get_graphs(state: State<'_, SpawnFileState>) -> TauriResult<Option<SpawnGraphsChunk>> {
  log::debug!("Getting spawn file graphs");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  Ok(lock.as_ref().map(|file| file.graphs.clone()))
}

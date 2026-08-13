use std::sync::MutexGuard;

use tauri::State;
use xrf_db::{SpawnALifeSpawnsChunk, SpawnFile};

use crate::spawn::state::SpawnFileState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_alife_spawns"))]
#[tauri::command(rename = "get_alife_spawns")]
pub async fn spawn_get_alife_spawns(state: State<'_, SpawnFileState>) -> TauriResult<Option<SpawnALifeSpawnsChunk>> {
  log::debug!("Getting spawn file alife spawns");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  Ok(lock.as_ref().map(|file| file.alife_spawn.clone()))
}

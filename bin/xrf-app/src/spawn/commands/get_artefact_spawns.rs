use std::sync::MutexGuard;

use tauri::State;
use xrf_db::{SpawnArtefactSpawnsChunk, SpawnFile};

use crate::spawn::state::SpawnFileState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_artefact_spawns"))]
#[tauri::command(rename = "get_artefact_spawns")]
pub async fn spawn_get_artefact_spawns(
  state: State<'_, SpawnFileState>,
) -> TauriResult<Option<SpawnArtefactSpawnsChunk>> {
  log::debug!("Getting spawn file artefact spawns");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  Ok(lock.as_ref().map(|file| file.artefact_spawn.clone()))
}

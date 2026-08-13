use std::sync::MutexGuard;

use tauri::State;
use xrf_db::{SpawnALifeSpawnsChunk, SpawnFile};

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn get_spawn_file_alife_spawns(
  state: State<'_, SpawnsEditorState>,
) -> TauriResult<Option<SpawnALifeSpawnsChunk>> {
  log::debug!("Getting spawn file alife spawns");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  Ok(lock.as_ref().map(|file| file.alife_spawn.clone()))
}

use std::sync::MutexGuard;

use tauri::State;
use xrf_db::{SpawnFile, SpawnGraphsChunk};

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn get_spawn_file_graphs(state: State<'_, SpawnsEditorState>) -> TauriResult<Option<SpawnGraphsChunk>> {
  log::debug!("Getting spawn file graphs");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  Ok(lock.as_ref().map(|file| file.graphs.clone()))
}

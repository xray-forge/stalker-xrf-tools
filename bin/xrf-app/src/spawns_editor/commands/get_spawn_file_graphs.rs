use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;
use serde_json::{json, Value};
use std::sync::MutexGuard;
use tauri::State;
use xray_db::SpawnFile;

#[tauri::command]
pub async fn get_spawn_file_graphs(
  state: State<'_, SpawnsEditorState>,
) -> TauriResult<Option<Value>> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap().graphs)))
}

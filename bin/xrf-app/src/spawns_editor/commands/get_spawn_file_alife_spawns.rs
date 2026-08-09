use std::sync::MutexGuard;

use serde_json::{Value, json};
use tauri::State;
use xray_db::SpawnFile;

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub async fn get_spawn_file_alife_spawns(state: State<'_, SpawnsEditorState>) -> TauriResult<Option<Value>> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap().alife_spawn)))
}

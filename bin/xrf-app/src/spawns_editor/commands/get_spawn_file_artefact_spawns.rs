use crate::spawns_editor::state::SpawnsEditorState;
use serde_json::{json, Value};
use std::sync::MutexGuard;
use tauri::State;
use xray_db::file::spawn_file::SpawnFile;

#[tauri::command]
pub async fn get_spawn_file_artefact_spawns(
  state: State<'_, SpawnsEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap().artefact_spawn)))
}

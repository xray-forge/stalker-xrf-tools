use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;
use std::sync::MutexGuard;
use tauri::State;
use xray_db::SpawnFile;

#[tauri::command]
pub fn close_spawn_file(state: State<'_, SpawnsEditorState>) -> TauriResult {
  log::info!("Closing spawn file");

  let mut lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  Ok(())
}

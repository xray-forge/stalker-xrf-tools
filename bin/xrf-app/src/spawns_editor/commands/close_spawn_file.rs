use crate::spawns_editor::state::SpawnsEditorState;
use std::sync::MutexGuard;
use tauri::State;
use xray_db::spawn_file::spawn_file::SpawnFile;

#[tauri::command]
pub fn close_spawn_file(state: State<'_, SpawnsEditorState>) {
  log::info!("Closing spawn file");

  let mut lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }
}

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;
use std::path::Path;
use std::sync::MutexGuard;
use tauri::State;
use xray_db::{SpawnFile, XRayByteOrder};

#[tauri::command]
pub fn save_spawn_file(path: &str, state: State<'_, SpawnsEditorState>) -> TauriResult {
  log::info!("Saving spawn file");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    let file: &SpawnFile = lock.as_ref().unwrap();

    file
      .write_to_path::<XRayByteOrder>(Path::new(path))
      .map_err(error_to_string)
  } else {
    Err(String::from("No spawn file open for saving"))
  }
}

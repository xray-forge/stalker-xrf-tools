use crate::spawns_editor::state::SpawnsEditorState;
use std::path::Path;
use std::sync::MutexGuard;
use tauri::State;
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

#[tauri::command]
pub fn save_spawn_file(path: &str, state: State<'_, SpawnsEditorState>) -> Result<(), String> {
  log::info!("Saving spawn file");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    let file: &SpawnFile = lock.as_ref().unwrap();

    match file.write_to_path::<SpawnByteOrder>(Path::new(path)) {
      Ok(_) => Ok(()),
      Err(error) => Err(error.to_string()),
    }
  } else {
    Err(String::from("No spawn file open for saving"))
  }
}

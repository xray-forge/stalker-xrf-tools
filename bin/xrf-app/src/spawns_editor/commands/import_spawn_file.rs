use std::path::Path;

use tauri::State;
use xrf_db::{SpawnFile, XRayByteOrder};

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub async fn import_spawn_file(path: &str, state: State<'_, SpawnsEditorState>) -> TauriResult<String> {
  log::info!("Importing spawn file");

  match SpawnFile::import_from_path::<XRayByteOrder, _>(&Path::new(path)) {
    Ok(file) => {
      log::info!("Imported spawn file");

      *state.file.lock().unwrap() = Some(file);
      *state.path.lock().unwrap() = Some(String::from(path));

      Ok(String::from("Imported spawn file"))
    }
    Err(error) => Err(format!("Failed to import provided spawn file path: {}", error)),
  }
}

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;
use std::path::Path;
use tauri::State;
use xray_db::{SpawnFile, XRayByteOrder};

#[tauri::command]
pub async fn import_spawn_file(
  path: &str,
  state: State<'_, SpawnsEditorState>,
) -> TauriResult<String> {
  log::info!("Importing spawn file");

  match SpawnFile::import_from_path::<XRayByteOrder>(Path::new(path)) {
    Ok(file) => {
      log::info!("Imported spawn file");

      *state.file.lock().unwrap() = Some(file);

      Ok(String::from("Imported spawn file"))
    }
    Err(error) => Err(format!(
      "Failed to import provided spawn file path: {}",
      error
    )),
  }
}

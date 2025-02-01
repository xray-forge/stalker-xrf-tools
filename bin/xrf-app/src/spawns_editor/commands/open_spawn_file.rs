use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;
use serde_json::{json, Value};
use std::path::Path;
use xray_db::{SpawnFile, XRayByteOrder};

#[tauri::command]
pub async fn open_spawn_file(
  path: &str,
  state: tauri::State<'_, SpawnsEditorState>,
) -> TauriResult<Value> {
  log::info!("Opening spawn file");

  match SpawnFile::read_from_path::<XRayByteOrder, &Path>(Path::new(path)) {
    Ok(file) => {
      log::info!("Opened spawn file");

      let json: Value = json!(file);

      *state.file.lock().unwrap() = Some(file);

      Ok(json)
    }
    Err(error) => Err(format!("Failed to open provided spawn file: {}", error)),
  }
}

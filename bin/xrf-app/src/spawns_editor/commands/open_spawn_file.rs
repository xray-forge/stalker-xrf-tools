use crate::spawns_editor::state::SpawnsEditorState;
use serde_json::{json, Value};
use std::path::Path;
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

#[tauri::command]
pub async fn open_spawn_file(
  path: &str,
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<Value, String> {
  log::info!("Opening spawn file");

  match SpawnFile::read_from_path::<SpawnByteOrder>(Path::new(path)) {
    Ok(file) => {
      log::info!("Opened spawn file");

      let json: Value = json!(file);

      *state.file.lock().unwrap() = Some(file);

      Ok(json)
    }
    Err(_) => Err(String::from("Failed to open provided spawn file")),
  }
}

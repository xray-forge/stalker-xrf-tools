use std::path::Path;

use serde_json::{Value, json};
use xray_db::{SpawnFile, XRayByteOrder};

use crate::spawns_editor::state::SpawnsEditorState;
use crate::types::TauriResult;

/// Read a packed spawn file into the editor session.
///
/// Answers with the header rather than the whole file: the UI reads chunks one at a time through the
/// per-chunk commands, so serialising every alife object here only to have it re-requested is waste
/// measured in tens of megabytes on a real all.spawn.
#[tauri::command]
pub async fn open_spawn_file(path: &str, state: tauri::State<'_, SpawnsEditorState>) -> TauriResult<Value> {
  log::info!("Opening spawn file");

  match SpawnFile::read_from_path::<XRayByteOrder, _>(&Path::new(path)) {
    Ok(file) => {
      log::info!("Opened spawn file");

      let header: Value = json!(file.header);

      *state.file.lock().unwrap() = Some(file);
      *state.path.lock().unwrap() = Some(String::from(path));

      Ok(header)
    }
    Err(error) => Err(format!("Failed to open provided spawn file: {}", error)),
  }
}

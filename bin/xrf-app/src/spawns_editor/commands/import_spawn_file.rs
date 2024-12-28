use crate::spawns_editor::state::SpawnsEditorState;
use std::path::Path;
use tauri::State;
use xray_db::spawn_file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

#[tauri::command]
pub async fn import_spawn_file(
  path: &str,
  state: State<'_, SpawnsEditorState>,
) -> Result<String, String> {
  log::info!("Importing spawn file");

  match SpawnFile::import_from_path::<SpawnByteOrder>(Path::new(path)) {
    Ok(file) => {
      log::info!("Imported spawn file");

      *state.file.lock().unwrap() = Some(file);

      Ok(String::from("Imported spawn file"))
    }
    Err(_) => Err(String::from("Failed to import provided spawn file pat")),
  }
}

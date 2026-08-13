use std::path::Path;

use tauri::State;
use xrf_db::{SpawnFile, XRayByteOrder};

use crate::spawn::state::SpawnFileState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_unpacked_directory"))]
#[tauri::command(rename = "open_unpacked_directory")]
pub async fn spawn_open_unpacked_directory(path: &str, state: State<'_, SpawnFileState>) -> TauriResult<String> {
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

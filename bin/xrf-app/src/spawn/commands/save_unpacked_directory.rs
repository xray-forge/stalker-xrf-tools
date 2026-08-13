use std::path::Path;
use std::sync::MutexGuard;

use tauri::State;
use xrf_db::{SpawnFile, XRayByteOrder};

use crate::spawn::state::SpawnFileState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "save_unpacked_directory"))]
#[tauri::command(rename = "save_unpacked_directory")]
pub async fn spawn_save_unpacked_directory(path: &str, state: State<'_, SpawnFileState>) -> TauriResult {
  log::info!("Saving spawn file");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    let file: &SpawnFile = lock.as_ref().unwrap();

    file
      .export_to_path::<XRayByteOrder, _>(&Path::new(path))
      .map_err(error_to_string)
  } else {
    Err(String::from("No spawn file open for saving"))
  }
}

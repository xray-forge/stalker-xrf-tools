use std::path::Path;
use std::sync::MutexGuard;

use tauri::State;
use xrf_db::{SpawnFile, XRayByteOrder};

use crate::app::types::TauriResult;
use crate::app::utils::error_to_string;
use crate::plugins::spawn::state::SpawnFileState;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "save_file"))]
#[tauri::command(rename = "save_file")]
pub fn spawn_save_file(path: &str, state: State<'_, SpawnFileState>) -> TauriResult {
  log::info!("Saving spawn file");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    let file: &SpawnFile = lock.as_ref().unwrap();

    file
      .write_to_path::<XRayByteOrder, _>(&Path::new(path))
      .map_err(error_to_string)
  } else {
    Err(String::from("No spawn file open for saving"))
  }
}

use tauri::State;

use crate::spawn::state::SpawnFileState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "has_file"))]
#[tauri::command(rename = "has_file")]
pub fn spawn_has_file(state: State<'_, SpawnFileState>) -> TauriResult<bool> {
  log::debug!("Checking spawn file presence");

  Ok(state.file.lock().unwrap().is_some())
}

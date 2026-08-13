use tauri::State;

use crate::archives::state::ArchiveProjectState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "has_project"))]
#[tauri::command(rename = "has_project")]
pub fn archives_has_project(state: State<'_, ArchiveProjectState>) -> TauriResult<bool> {
  log::debug!("Checking archives project presence");

  Ok(state.project.lock().unwrap().is_some())
}

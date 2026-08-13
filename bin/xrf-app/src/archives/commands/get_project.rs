use std::sync::MutexGuard;

use tauri::State;
use xrf_archive::ArchiveProject;

use crate::archives::state::ArchiveProjectState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_project"))]
#[tauri::command(rename = "get_project")]
pub async fn archives_get_project(state: State<'_, ArchiveProjectState>) -> TauriResult<Option<ArchiveProject>> {
  log::debug!("Getting archives project");

  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  Ok(lock.clone())
}

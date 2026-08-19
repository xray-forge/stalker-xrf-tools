use std::sync::MutexGuard;

use tauri::State;
use xrf_vfs::ArchiveProject;

use crate::archives::state::ArchiveProjectState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "close_project"))]
#[tauri::command(rename = "close_project")]
pub fn archives_close_project(state: State<'_, ArchiveProjectState>) -> TauriResult {
  log::info!("Closing archives project");

  let mut lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  Ok(())
}

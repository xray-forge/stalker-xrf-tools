use std::sync::MutexGuard;

use tauri::State;
use xrf_archive::ArchiveProject;

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub fn close_archives_project(state: State<'_, ArchivesEditorState>) -> TauriResult {
  log::info!("Closing archives project");

  let mut lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  Ok(())
}

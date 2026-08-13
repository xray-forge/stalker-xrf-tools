use std::sync::MutexGuard;

use tauri::State;
use xrf_archive::ArchiveProject;

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn get_archives_project(state: State<'_, ArchivesEditorState>) -> TauriResult<Option<ArchiveProject>> {
  log::debug!("Getting archives project");

  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  Ok(lock.clone())
}

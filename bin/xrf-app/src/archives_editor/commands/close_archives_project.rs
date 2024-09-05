use crate::archives_editor::state::ArchivesEditorState;
use std::sync::MutexGuard;
use tauri::State;
use xray_archive::ArchiveProject;

#[tauri::command]
pub fn close_archives_project(state: State<'_, ArchivesEditorState>) {
  log::info!("Closing archives project");

  let mut lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }
}

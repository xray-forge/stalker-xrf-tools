use crate::archives_editor::state::ArchivesEditorState;
use serde_json::{json, Value};
use std::sync::MutexGuard;
use tauri::State;
use xray_archive::ArchiveProject;

#[tauri::command]
pub async fn get_archives_project(
  state: State<'_, ArchivesEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if (*lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap())))
}

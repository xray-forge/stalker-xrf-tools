use std::sync::MutexGuard;

use serde_json::{Value, json};
use tauri::State;
use xray_archive::ArchiveProject;

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub async fn get_archives_project(
  state: State<'_, ArchivesEditorState>,
) -> TauriResult<Option<Value>> {
  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if (*lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap())))
}

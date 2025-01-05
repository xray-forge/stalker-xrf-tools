use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;
use serde_json::{json, Value};
use std::sync::MutexGuard;
use tauri::State;
use xray_archive::ArchiveProject;

#[tauri::command]
pub async fn read_archive_file(
  path: &str,
  state: State<'_, ArchivesEditorState>,
) -> TauriResult<Value> {
  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if (*lock).is_none() {
    return Err(String::from("Failed to read file - archive is not open"));
  }

  lock
    .as_ref()
    .unwrap()
    .read_file_as_string(path)
    .map(|result| json!(result))
    .map_err(|error| error.to_string())
}

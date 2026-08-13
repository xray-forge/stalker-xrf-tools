use std::sync::MutexGuard;

use tauri::State;
use xrf_archive::{ArchiveProject, ProjectReadResult};

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn read_archive_file(path: &str, state: State<'_, ArchivesEditorState>) -> TauriResult<ProjectReadResult> {
  log::info!("Reading archive file: {}", path);

  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if (*lock).is_none() {
    return Err(String::from("Failed to read file - archive is not open"));
  }

  lock
    .as_ref()
    .unwrap()
    .read_file_as_string(path)
    .map_err(|error| error.to_string())
}

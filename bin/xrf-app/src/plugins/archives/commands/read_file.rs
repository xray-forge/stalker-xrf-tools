use std::sync::MutexGuard;

use tauri::State;
use xrf_volume::{ArchiveProject, ProjectReadResult};

use crate::core::types::TauriResult;
use crate::plugins::archives::state::ArchiveProjectState;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "read_file"))]
#[tauri::command(rename = "read_file")]
pub async fn archives_read_file(path: &str, state: State<'_, ArchiveProjectState>) -> TauriResult<ProjectReadResult> {
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

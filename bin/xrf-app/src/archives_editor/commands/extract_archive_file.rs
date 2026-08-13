use std::sync::MutexGuard;

use tauri::State;
use xrf_archive::{ArchiveExtractResult, ArchiveProject};

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

/// Write a single archived file to a path the user chose.
#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn extract_archive_file(
  name: &str,
  destination: &str,
  state: State<'_, ArchivesEditorState>,
) -> TauriResult<ArchiveExtractResult> {
  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  let project: &ArchiveProject = lock
    .as_ref()
    .ok_or_else(|| String::from("Failed to extract file - archive is not open"))?;

  log::info!("Extracting archive file '{}' to '{}'", name, destination);

  let result: ArchiveExtractResult = project
    .extract_file(name, destination)
    .map_err(|error| error.to_string())?;

  Ok(result)
}

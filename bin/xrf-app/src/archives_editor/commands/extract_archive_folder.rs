use std::sync::MutexGuard;

use tauri::State;
use xrf_archive::{ArchiveExtractFolderResult, ArchiveProject};

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

/// Write every archived file under one directory into a destination root.
///
/// An empty prefix means the whole archive, so this also covers extracting everything without needing
/// a separate command.
#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn extract_archive_folder(
  prefix: &str,
  destination: &str,
  state: State<'_, ArchivesEditorState>,
) -> TauriResult<ArchiveExtractFolderResult> {
  let lock: MutexGuard<Option<ArchiveProject>> = state
    .project
    .lock()
    .map_err(|error| format!("Failed to extract folder - archive state is unavailable: {error}"))?;

  let project: &ArchiveProject = lock
    .as_ref()
    .ok_or_else(|| String::from("Failed to extract folder - archive is not open"))?;

  log::info!("Extracting archive folder '{}' to '{}'", prefix, destination);

  let result: ArchiveExtractFolderResult = project
    .extract_folder(prefix, destination)
    .map_err(|error| error.to_string())?;

  Ok(result)
}

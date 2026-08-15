use std::sync::MutexGuard;

use tauri::State;
use xrf_archive::{ArchiveExtractDirectoryResult, ArchiveProject, ArchiveUnpacker};

use crate::archives::state::ArchiveProjectState;
use crate::types::TauriResult;

/// Write every archived file under one directory into a destination root.
///
/// An empty prefix means the whole archive, so this also covers extracting everything without needing
/// a separate command.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "extract_directory"))]
#[tauri::command(rename = "extract_directory")]
pub async fn archives_extract_directory(
  prefix: &str,
  destination: &str,
  state: State<'_, ArchiveProjectState>,
) -> TauriResult<ArchiveExtractDirectoryResult> {
  let lock: MutexGuard<Option<ArchiveProject>> = state
    .project
    .lock()
    .map_err(|error| format!("Failed to extract directory - archive state is unavailable: {error}"))?;

  let project: &ArchiveProject = lock
    .as_ref()
    .ok_or_else(|| String::from("Failed to extract directory - archive is not open"))?;

  log::info!("Extracting archive directory '{}' to '{}'", prefix, destination);

  let result: ArchiveExtractDirectoryResult =
    ArchiveUnpacker::extract_directory(project, prefix, destination).map_err(|error| error.to_string())?;

  Ok(result)
}

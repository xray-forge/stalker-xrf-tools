use std::sync::MutexGuard;

use tauri::State;
use xrf_archive::{ArchiveExtractResult, ArchiveUnpacker};
use xrf_vfs::ArchiveProject;

use crate::app::types::TauriResult;
use crate::plugins::archives::state::ArchiveProjectState;

/// Write a single archived file to a path the user chose.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "extract_file"))]
#[tauri::command(rename = "extract_file")]
pub async fn archives_extract_file(
  name: &str,
  destination: &str,
  state: State<'_, ArchiveProjectState>,
) -> TauriResult<ArchiveExtractResult> {
  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  let project: &ArchiveProject = lock
    .as_ref()
    .ok_or_else(|| String::from("Failed to extract file - archive is not open"))?;

  log::info!("Extracting archive file '{}' to '{}'", name, destination);

  let result: ArchiveExtractResult =
    ArchiveUnpacker::extract_file(project, name, destination).map_err(|error| error.to_string())?;

  Ok(result)
}

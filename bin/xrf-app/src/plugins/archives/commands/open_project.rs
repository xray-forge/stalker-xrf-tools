use std::path::Path;

use tauri::State;
use xrf_archive::ArchiveProject;

use crate::core::types::TauriResult;
use crate::plugins::archives::state::ArchiveProjectState;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_project"))]
#[tauri::command(rename = "open_project")]
pub async fn archives_open_project(path: &str, state: State<'_, ArchiveProjectState>) -> TauriResult<ArchiveProject> {
  log::info!("Opening archives project");

  let project: ArchiveProject = ArchiveProject::new(Path::new(path))
    .map_err(|error| format!("Failed to open provided archive project: {}", error))?;

  log::info!("Opened archives project");

  *state.project.lock().unwrap() = Some(project.clone());

  Ok(project)
}

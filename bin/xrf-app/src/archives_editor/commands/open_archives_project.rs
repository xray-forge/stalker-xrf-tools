use std::path::Path;

use tauri::State;
use xrf_archive::ArchiveProject;

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn open_archives_project(path: &str, state: State<'_, ArchivesEditorState>) -> TauriResult<ArchiveProject> {
  log::info!("Opening archives project");

  let project: ArchiveProject = ArchiveProject::new(&Path::new(path))
    .map_err(|error| format!("Failed to open provided archive project: {}", error))?;

  log::info!("Opened archives project");

  *state.project.lock().unwrap() = Some(project.clone());

  Ok(project)
}

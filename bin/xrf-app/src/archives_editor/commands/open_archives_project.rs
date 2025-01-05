use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;
use serde_json::{json, Value};
use std::path::Path;
use tauri::State;
use xray_archive::ArchiveProject;

#[tauri::command]
pub async fn open_archives_project(
  path: &str,
  state: State<'_, ArchivesEditorState>,
) -> TauriResult<Value> {
  log::info!("Opening archives project");

  let project: ArchiveProject = ArchiveProject::new(Path::new(path))
    .map_err(|error| format!("Failed to open provided archive project: {}", error))?;

  log::info!("Opened archives project");

  let json: Value = json!(project);

  *state.project.lock().unwrap() = Some(project);

  Ok(json)
}

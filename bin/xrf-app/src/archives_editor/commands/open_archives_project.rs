use crate::archives_editor::state::ArchivesEditorState;
use serde_json::{json, Value};
use std::path::Path;
use tauri::State;
use xray_archive::ArchiveProject;

#[tauri::command]
pub async fn open_archives_project(
  path: &str,
  state: State<'_, ArchivesEditorState>,
) -> Result<Value, String> {
  log::info!("Opening archives project");

  match ArchiveProject::new(Path::new(path)) {
    Ok(project) => {
      log::info!("Opened archives project");

      let json: Value = json!(project);

      *state.project.lock().unwrap() = Some(project);

      Ok(json)
    }
    Err(_) => Err(String::from("Failed to open provided archive project")),
  }
}

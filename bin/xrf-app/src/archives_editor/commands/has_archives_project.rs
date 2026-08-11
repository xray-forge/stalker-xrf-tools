use tauri::State;

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub fn has_archives_project(state: State<'_, ArchivesEditorState>) -> TauriResult<bool> {
  log::debug!("Checking archives project presence");

  Ok(state.project.lock().unwrap().is_some())
}

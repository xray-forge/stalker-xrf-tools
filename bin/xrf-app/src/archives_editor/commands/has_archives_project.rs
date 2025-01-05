use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;
use tauri::State;

#[tauri::command]
pub fn has_archives_project(state: State<'_, ArchivesEditorState>) -> TauriResult<bool> {
  Ok(state.project.lock().unwrap().is_some())
}

use crate::archives_editor::state::ArchivesEditorState;
use tauri::State;

#[tauri::command]
pub fn has_archives_project(state: State<'_, ArchivesEditorState>) -> bool {
  state.project.lock().unwrap().is_some()
}

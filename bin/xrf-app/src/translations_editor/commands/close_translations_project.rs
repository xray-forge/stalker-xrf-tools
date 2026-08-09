use tauri::State;

use crate::translations_editor::state::TranslationsEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub async fn close_translations_project(state: State<'_, TranslationsEditorState>) -> TauriResult {
  log::info!("Closing translations project");

  *state.project.lock().unwrap() = None;

  Ok(())
}

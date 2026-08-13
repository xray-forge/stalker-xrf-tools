use tauri::State;

use crate::translations_editor::state::TranslationsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn close_translations_project(state: State<'_, TranslationsEditorState>) -> TauriResult {
  log::info!("Closing translations project");

  *state.project.lock().unwrap() = None;

  Ok(())
}

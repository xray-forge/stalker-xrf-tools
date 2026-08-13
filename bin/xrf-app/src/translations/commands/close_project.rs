use tauri::State;

use crate::translations::state::TranslationProjectState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "close_project"))]
#[tauri::command(rename = "close_project")]
pub async fn translations_close_project(state: State<'_, TranslationProjectState>) -> TauriResult {
  log::info!("Closing translations project");

  *state.project.lock().unwrap() = None;

  Ok(())
}

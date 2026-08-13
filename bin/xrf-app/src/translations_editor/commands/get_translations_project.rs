use std::sync::MutexGuard;

use tauri::State;
use xrf_translation::TranslationProjectJson;

use crate::translations_editor::state::TranslationsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn get_translations_project(
  state: State<'_, TranslationsEditorState>,
) -> TauriResult<Option<TranslationProjectJson>> {
  log::info!("Getting translations project");

  let lock: MutexGuard<Option<TranslationProjectJson>> = state.project.lock().unwrap();

  Ok(lock.clone())
}

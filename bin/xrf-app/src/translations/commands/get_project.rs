use std::sync::MutexGuard;

use tauri::State;
use xrf_translation::TranslationProjectDescriptor;

use crate::translations::state::TranslationProjectState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_project"))]
#[tauri::command(rename = "get_project")]
pub async fn translations_get_project(
  state: State<'_, TranslationProjectState>,
) -> TauriResult<Option<TranslationProjectDescriptor>> {
  log::info!("Getting translations project");

  let lock: MutexGuard<Option<TranslationProjectDescriptor>> = state.project.lock().unwrap();

  Ok(lock.clone())
}

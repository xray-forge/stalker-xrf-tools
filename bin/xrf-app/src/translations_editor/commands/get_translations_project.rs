use crate::translations_editor::state::TranslationsEditorState;
use crate::types::TauriResult;
use serde_json::{json, Value};
use std::sync::MutexGuard;
use tauri::State;
use xray_translation::TranslationProjectJson;

#[tauri::command]
pub async fn get_translations_project(
  state: State<'_, TranslationsEditorState>,
) -> TauriResult<Option<Value>> {
  log::info!("Getting translations project");

  let lock: MutexGuard<Option<TranslationProjectJson>> = state.project.lock().unwrap();

  Ok(lock.as_ref().map(|it| json!(it)))
}

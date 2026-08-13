use serde_json::{Value, json};
use tauri::State;
use xrf_translation::{TranslationProject, TranslationProjectJson};

use crate::translations_editor::state::TranslationsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[tauri::command]
pub async fn read_translations_project(path: &str, _: State<'_, TranslationsEditorState>) -> TauriResult<Value> {
  log::info!("Reading translations project: {}", path);

  let value: TranslationProjectJson = TranslationProject::read_project(path).map_err(error_to_string)?;

  Ok(json!(value))
}

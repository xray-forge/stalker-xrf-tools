use crate::translations_editor::state::TranslationsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::State;
use xray_translation::{TranslationProject, TranslationProjectJson};

#[tauri::command]
pub async fn open_translations_project(
  path: &str,
  state: State<'_, TranslationsEditorState>,
) -> TauriResult<Value> {
  log::info!("Opening translations project: {:?}", path);

  let translation: TranslationProjectJson =
    TranslationProject::read_project(&PathBuf::from(path)).map_err(error_to_string)?;
  let response: Value = json!(translation);

  *state.project.lock().unwrap() = Some(translation);

  Ok(response)
}

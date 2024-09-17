use crate::translations_editor::state::TranslationsEditorState;
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::State;
use xray_translation::TranslationProject;

#[tauri::command]
pub async fn read_translations_project(
  path: &str,
  _: State<'_, TranslationsEditorState>,
) -> Result<Value, String> {
  log::info!("Reading translations project: {:?}", path);

  TranslationProject::read_project(&PathBuf::from(path))
    .map(|value| json!(value))
    .map_err(|error| error.to_string())
}

use crate::translations_editor::state::TranslationsEditorState;
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::State;
use xray_translation::TranslationProject;

#[tauri::command]
pub async fn open_translations_project(
  path: &str,
  state: State<'_, TranslationsEditorState>,
) -> Result<Value, String> {
  log::info!("Opening translations project: {:?}", path);

  match TranslationProject::read_project(&PathBuf::from(path)) {
    Ok(translation) => {
      let response: Value = json!(translation);

      *state.project.lock().unwrap() = Some(translation);

      Ok(response)
    }
    Err(error) => Err(error.to_string()),
  }
}

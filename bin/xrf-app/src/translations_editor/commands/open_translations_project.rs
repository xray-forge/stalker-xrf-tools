use tauri::State;
use xrf_translation::{TranslationProject, TranslationProjectJson};

use crate::translations_editor::state::TranslationsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn open_translations_project(
  path: &str,
  state: State<'_, TranslationsEditorState>,
) -> TauriResult<TranslationProjectJson> {
  log::info!("Opening translations project: {}", path);

  let translation: TranslationProjectJson = TranslationProject::read_project(path).map_err(error_to_string)?;
  *state.project.lock().unwrap() = Some(translation.clone());

  Ok(translation)
}

use tauri::State;
use xrf_translation::{TranslationProject, TranslationProjectJson};

use crate::translations::state::TranslationProjectState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_project"))]
#[tauri::command(rename = "open_project")]
pub async fn translations_open_project(
  path: &str,
  state: State<'_, TranslationProjectState>,
) -> TauriResult<TranslationProjectJson> {
  log::info!("Opening translations project: {}", path);

  let translation: TranslationProjectJson = TranslationProject::read_project(path).map_err(error_to_string)?;
  *state.project.lock().unwrap() = Some(translation.clone());

  Ok(translation)
}

use tauri::State;
use xrf_translation::{TranslationProject, TranslationProjectJson};

use crate::translations::state::TranslationProjectState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "read_project"))]
#[tauri::command(rename = "read_project")]
pub async fn translations_read_project(
  path: &str,
  _state: State<'_, TranslationProjectState>,
) -> TauriResult<TranslationProjectJson> {
  log::info!("Reading translations project: {}", path);

  let value: TranslationProjectJson = TranslationProject::read_project(path).map_err(error_to_string)?;

  Ok(value)
}

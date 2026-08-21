use tauri::State;
use xrf_translation::{TranslationProjectDescriptor, TranslationProjectMode, read_gamedata, read_source};

use crate::core::error::error_to_string;
use crate::core::types::TauriResult;
use crate::plugins::translations::state::TranslationProjectState;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_project"))]
#[tauri::command(rename = "open_project")]
pub async fn translations_open_project(
  path: &str,
  mode: TranslationProjectMode,
  state: State<'_, TranslationProjectState>,
) -> TauriResult<TranslationProjectDescriptor> {
  log::info!("Opening translations project: {} ({:?})", path, mode);

  // The caller's mode is obeyed, not re-derived: the two layouts save to different files, so a guess
  // acted on here would decide what a later save overwrites.
  let descriptor: TranslationProjectDescriptor = match mode {
    TranslationProjectMode::Source => read_source(path),
    TranslationProjectMode::Gamedata => read_gamedata(path),
  }
  .map_err(error_to_string)?;

  *state.project.lock().unwrap() = Some(descriptor.clone());

  Ok(descriptor)
}

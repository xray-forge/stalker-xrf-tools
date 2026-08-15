use std::path::Path;

use xrf_translation::{TranslationProjectMode, detect_mode};

use crate::types::TauriResult;

/// Report which layout a directory looks like, for the open form to preselect.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "detect_mode"))]
#[tauri::command(rename = "detect_mode")]
pub async fn translations_detect_mode(path: &str) -> TauriResult<TranslationProjectMode> {
  let mode: TranslationProjectMode = detect_mode(Path::new(path));

  log::info!("Detected translations layout at {}: {:?}", path, mode);

  Ok(mode)
}

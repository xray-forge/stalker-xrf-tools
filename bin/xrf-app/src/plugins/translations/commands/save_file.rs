use std::collections::HashMap;
use std::path::Path;

use tauri::State;
use xrf_translation::{
  TranslationEdit, TranslationFile, TranslationProjectDescriptor, TranslationProjectMode, apply_edits, read_gamedata,
  read_source,
};

use crate::app::types::TauriResult;
use crate::app::utils::error_to_string;
use crate::plugins::translations::state::TranslationProjectState;

/// Write one logical file's pending edits, grouped by the language each belongs to.
///
/// A logical file is several files on disk in gamedata mode, one per language, so the edits arrive
/// keyed by language and each group goes to its own path. The paths come from the open project rather
/// than from the caller, so a save can only ever touch files this project actually read.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "save_file"))]
#[tauri::command(rename = "save_file")]
pub async fn translations_save_file(
  file: &str,
  edits: HashMap<String, Vec<TranslationEdit>>,
  state: State<'_, TranslationProjectState>,
) -> TauriResult<TranslationProjectDescriptor> {
  let (root, mode, sources) = {
    let lock = state.project.lock().unwrap();
    let descriptor: &TranslationProjectDescriptor = lock
      .as_ref()
      .ok_or_else(|| String::from("No translations project is open"))?;
    let entry: &TranslationFile = descriptor
      .files
      .get(file)
      .ok_or_else(|| format!("Translations file '{file}' is not part of the open project"))?;

    (descriptor.root.clone(), descriptor.mode, entry.sources.clone())
  };

  for (language, language_edits) in &edits {
    if language_edits.is_empty() {
      continue;
    }

    let path: &String = sources
      .get(language)
      .ok_or_else(|| format!("Translations file '{file}' has nothing on disk for '{language}'"))?;

    log::info!("Saving {} edits to {} ({})", language_edits.len(), path, language);

    apply_edits(Path::new(path), language, language_edits).map_err(error_to_string)?;
  }

  // Re-read rather than patch the cached copy: what is on disk now is the only version worth showing,
  // and a write can add or drop entries the caller did not predict.
  let refreshed: TranslationProjectDescriptor = match mode {
    TranslationProjectMode::Source => read_source(&root),
    TranslationProjectMode::Gamedata => read_gamedata(&root),
  }
  .map_err(error_to_string)?;

  *state.project.lock().unwrap() = Some(refreshed.clone());

  Ok(refreshed)
}

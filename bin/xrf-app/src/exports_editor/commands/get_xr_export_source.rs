use serde_json::{Value, json};
use tauri::State;
use xrf_export::{ExportSourceContent, ExportsProject};

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[tauri::command]
pub async fn get_xr_export_source(name: &str, state: State<'_, ExportsEditorState>) -> TauriResult<Value> {
  log::info!("Reading source of xr export: {name}");

  // Cloned rather than read under the lock: reading the file is IO, and the state is shared with every
  // other command the editor may be running.
  let project: Option<ExportsProject> = state.project.lock().unwrap().as_ref().cloned();

  let Some(project) = project else {
    return Err(String::from("No exports project is open."));
  };

  let source: ExportSourceContent = project.read_declaration_source(name).map_err(error_to_string)?;

  Ok(json!(source))
}

use tauri::State;
use xrf_export::{ExportSourceContent, ExportsProject};

use crate::core::error::error_to_string;
use crate::core::types::TauriResult;
use crate::plugins::exports::state::ExportsProjectState;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_source"))]
#[tauri::command(rename = "get_source")]
pub async fn exports_get_source(name: &str, state: State<'_, ExportsProjectState>) -> TauriResult<ExportSourceContent> {
  log::info!("Reading source of xr export: {name}");

  // Cloned rather than read under the lock: reading the file is IO, and the state is shared with every
  // other command the application may be running.
  let project: Option<ExportsProject> = state.project.lock().unwrap().as_ref().cloned();

  let Some(project) = project else {
    return Err(String::from("No exports project is open."));
  };

  let source: ExportSourceContent = project.read_declaration_source(name).map_err(error_to_string)?;

  Ok(source)
}

use tauri::State;
use xrf_export::{ExportsEditorParser, ExportsProject};

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn open_xr_exports(project_path: &str, state: State<'_, ExportsEditorState>) -> TauriResult<ExportsProject> {
  log::info!("Parsing externs from project: {project_path}");

  let parser: ExportsEditorParser = ExportsEditorParser::new();
  let project: ExportsProject = parser.parse_project_from_path(project_path).map_err(error_to_string)?;
  *state.project.lock().unwrap() = Some(project.clone());

  Ok(project)
}

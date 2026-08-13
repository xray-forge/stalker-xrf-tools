use serde_json::{Value, json};
use tauri::State;
use xrf_export::{ExportsEditorParser, ExportsProject};

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[tauri::command]
pub async fn open_xr_exports(project_path: &str, state: State<'_, ExportsEditorState>) -> TauriResult<Value> {
  log::info!("Parsing externs from project: {project_path}");

  let parser: ExportsEditorParser = ExportsEditorParser::new();
  let project: ExportsProject = parser.parse_project_from_path(project_path).map_err(error_to_string)?;
  let json: Value = json!(&project);

  *state.project.lock().unwrap() = Some(project);

  Ok(json)
}

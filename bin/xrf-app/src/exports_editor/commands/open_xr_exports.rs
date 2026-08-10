use serde_json::{Value, json};
use tauri::State;
use xray_export::{ExportDescriptor, ExportsEditorParser};

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[tauri::command]
pub async fn open_xr_exports(project_path: &str, state: State<'_, ExportsEditorState>) -> TauriResult<Value> {
  log::info!("Parsing externs from project: {project_path}");

  let parser: ExportsEditorParser = ExportsEditorParser::new();
  let declarations: Vec<ExportDescriptor> = parser.parse_project_from_path(project_path).map_err(error_to_string)?;
  let json: Value = json!(&declarations);

  *state.exports.lock().unwrap() = Some(declarations);

  Ok(json)
}

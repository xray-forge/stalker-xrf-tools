use serde_json::{Value, json};
use tauri::State;
use xray_export::ExportsProject;

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub async fn get_xr_exports(state: State<'_, ExportsEditorState>) -> TauriResult<Option<Value>> {
  log::debug!("Getting xr exports");

  let project: Option<ExportsProject> = state.project.lock().unwrap().as_ref().cloned();

  Ok(project.map(|project: ExportsProject| json!(project)))
}

use tauri::State;
use xrf_export::ExportsProject;

use crate::app::types::TauriResult;
use crate::plugins::exports::state::ExportsProjectState;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_project"))]
#[tauri::command(rename = "get_project")]
pub async fn exports_get_project(state: State<'_, ExportsProjectState>) -> TauriResult<Option<ExportsProject>> {
  log::debug!("Getting xr exports");

  let project: Option<ExportsProject> = state.project.lock().unwrap().as_ref().cloned();

  Ok(project)
}

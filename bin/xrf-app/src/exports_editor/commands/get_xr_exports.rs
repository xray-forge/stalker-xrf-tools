use tauri::State;
use xrf_export::ExportsProject;

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn get_xr_exports(state: State<'_, ExportsEditorState>) -> TauriResult<Option<ExportsProject>> {
  log::debug!("Getting xr exports");

  let project: Option<ExportsProject> = state.project.lock().unwrap().as_ref().cloned();

  Ok(project)
}

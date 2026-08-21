use std::sync::MutexGuard;

use tauri::State;
use xrf_export::ExportsProject;

use crate::app::types::TauriResult;
use crate::plugins::exports::state::ExportsProjectState;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "close_project"))]
#[tauri::command(rename = "close_project")]
pub fn exports_close_project(state: State<'_, ExportsProjectState>) -> TauriResult {
  log::info!("Closing xr exports");

  let mut lock: MutexGuard<Option<ExportsProject>> = state.project.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  Ok(())
}

use std::sync::MutexGuard;

use tauri::State;
use xrf_export::ExportsProject;

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub fn close_xr_exports(state: State<'_, ExportsEditorState>) -> TauriResult {
  log::info!("Closing xr exports");

  let mut lock: MutexGuard<Option<ExportsProject>> = state.project.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  Ok(())
}

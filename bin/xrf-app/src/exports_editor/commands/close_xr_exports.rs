use std::sync::MutexGuard;

use tauri::State;
use xray_export::ExportDescriptor;

use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;

#[tauri::command]
pub fn close_xr_exports(state: State<'_, ExportsEditorState>) -> TauriResult {
  log::info!("Closing xr exports");

  let mut lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.exports.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  Ok(())
}

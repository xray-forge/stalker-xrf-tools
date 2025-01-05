use crate::exports_editor::state::ExportsEditorState;
use crate::types::TauriResult;
use std::sync::MutexGuard;
use tauri::State;
use xray_export::ExportDescriptor;

#[tauri::command]
pub fn close_xr_effects(state: State<'_, ExportsEditorState>) -> TauriResult {
  log::info!("Closing xr effects project");

  let mut lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.effects.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }

  Ok(())
}

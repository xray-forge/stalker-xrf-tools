use crate::exports_editor::state::ExportsEditorState;
use std::sync::MutexGuard;
use tauri::State;
use xray_export::ExportDescriptor;

#[tauri::command]
pub fn close_xr_effects(state: State<'_, ExportsEditorState>) {
  log::info!("Closing xr effects project");

  let mut lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.effects.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }
}

use std::sync::MutexGuard;

use tauri::State;

use crate::core::types::TauriResult;
use crate::plugins::visuals::state::{SelectedVisual, VisualState};

/// Drop the selected visual and its packed geometry.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "close_model"))]
#[tauri::command(rename = "close_model")]
pub async fn visuals_close_model(state: State<'_, VisualState>) -> TauriResult {
  log::info!("Closing selected visual");

  let mut selected: MutexGuard<Option<SelectedVisual>> = state
    .selected
    .lock()
    .map_err(|error| format!("Failed to close visual - selection state is unavailable: {error}"))?;

  *selected = None;

  Ok(())
}

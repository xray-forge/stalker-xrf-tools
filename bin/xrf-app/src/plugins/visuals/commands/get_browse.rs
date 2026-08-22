use std::sync::MutexGuard;

use tauri::State;

use crate::core::assets::AssetWorldSpec;
use crate::core::types::TauriResult;
use crate::plugins::visuals::state::VisualState;

/// The world the viewer was browsing, or null when it was showing one visual on its own.
///
/// The rehydration probe for the tree, beside the one the selection already has: a reloaded frontend asks what is
/// being browsed and lists it again, so the panel comes back rather than emptying beside a model still open.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "get_browse"))]
#[tauri::command(rename = "get_browse")]
pub async fn visuals_get_browse(state: State<'_, VisualState>) -> TauriResult<Option<AssetWorldSpec>> {
  let browsed: MutexGuard<Option<AssetWorldSpec>> = state
    .browsed
    .lock()
    .map_err(|error| format!("Failed to read browse state: {error}"))?;

  match browsed.as_ref() {
    Some(world) => {
      log::info!("Reporting browsed world: {}", world.roots.join(", "));

      Ok(Some(world.clone()))
    }
    None => {
      log::info!("Reporting no browsed world");

      Ok(None)
    }
  }
}

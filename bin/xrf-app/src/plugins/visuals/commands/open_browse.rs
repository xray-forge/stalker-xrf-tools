use std::sync::MutexGuard;

use tauri::State;

use crate::core::assets::AssetWorldSpec;
use crate::core::types::TauriResult;
use crate::plugins::visuals::state::VisualState;

/// Start browsing a world of visuals.
///
/// Stores the intent rather than a listing: what the user chose is the world, and everything shown of it is derived
/// from that through the generic asset listing. A reload asks for this and derives the rest again.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_browse"))]
#[tauri::command(rename = "open_browse")]
pub async fn visuals_open_browse(world: AssetWorldSpec, state: State<'_, VisualState>) -> TauriResult {
  log::info!("Browsing visuals in: {}", world.roots.join(", "));

  let mut browsed: MutexGuard<Option<AssetWorldSpec>> = state
    .browsed
    .lock()
    .map_err(|error| format!("Failed to browse visuals - browse state is unavailable: {error}"))?;

  *browsed = Some(world);

  Ok(())
}

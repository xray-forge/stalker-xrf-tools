use std::sync::MutexGuard;

use tauri::State;
use tauri::ipc::Response;
use xrf_visual::VisualPackage;

use crate::app::types::TauriResult;
use crate::plugins::visuals::read::pack_source;
use crate::plugins::visuals::state::{SelectedVisual, VisualSource, VisualState};

/// The packed attribute buffers of a visual, as bytes.
///
/// Addressed by source rather than by the selection, so a response cannot be paired with a different
/// model's description when a user clicks through several in a row. Serving the selected model comes out
/// of the parse `open_model` already did; anything else is read on the spot and changes no state.
#[tauri::command(rename = "read_geometry")]
pub async fn visuals_read_geometry(source: VisualSource, state: State<'_, VisualState>) -> TauriResult<Response> {
  log::info!("Reading visual geometry: {}", source.label());

  let selected: MutexGuard<Option<SelectedVisual>> = state
    .selected
    .lock()
    .map_err(|error| format!("Failed to read geometry - selection state is unavailable: {error}"))?;

  if let Some(current) = selected.as_ref().filter(|it| it.source == source) {
    let buffer: Vec<u8> = current.package.buffer.clone();

    log::info!("Serving {} bytes of parked geometry", buffer.len());

    return Ok(Response::new(buffer));
  }

  drop(selected);

  let package: VisualPackage = pack_source(&source)?;

  log::info!(
    "Serving {} bytes of freshly packed geometry, {} submeshes",
    package.buffer.len(),
    package.description.submeshes.len()
  );

  Ok(Response::new(package.buffer))
}

use std::sync::MutexGuard;

use tauri::State;
use xrf_visual::VisualPackage;

use crate::types::TauriResult;
use crate::visuals::read::pack_source;
use crate::visuals::state::{SelectedVisual, SelectedVisualDescription, VisualSource, VisualState};

/// Select a visual and return what it contains.
///
/// Geometry is packed here and parked, so the `read_geometry` that follows serves the same parse rather
/// than repeating it. The bytes are not returned: a typed command cannot carry them, which is why they
/// are read separately.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_model"))]
#[tauri::command(rename = "open_model")]
pub async fn visuals_open_model(
  source: VisualSource,
  state: State<'_, VisualState>,
) -> TauriResult<SelectedVisualDescription> {
  log::info!("Opening visual: {}", source.label());

  let package: VisualPackage = pack_source(&source)?;
  let description: SelectedVisualDescription = SelectedVisualDescription {
    source: source.clone(),
    description: package.description.clone(),
  };

  let mut selected: MutexGuard<Option<SelectedVisual>> = state
    .selected
    .lock()
    .map_err(|error| format!("Failed to open visual - selection state is unavailable: {error}"))?;

  *selected = Some(SelectedVisual { source, package });

  Ok(description)
}

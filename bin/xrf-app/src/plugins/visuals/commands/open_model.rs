use std::sync::MutexGuard;

use tauri::State;
use xrf_visual::{VisualDependencies, VisualPackage};

use crate::core::assets::{AssetWorldSpec, AssetWorldState};
use crate::core::types::TauriResult;
use crate::plugins::visuals::read::pack_source;
use crate::plugins::visuals::state::{SelectedVisual, SelectedVisualDescription, VisualSource, VisualState};

/// Select a visual and return what it contains, with every reference it declares resolved.
///
/// Geometry is packed here and parked, so the `read_geometry` that follows serves the same parse rather than repeating
/// it. The bytes are not returned: a typed command cannot carry them, which is why they are read separately.
///
/// Resolution happens once, for the whole dependency set, in this one call. That is what keeps a model with forty
/// textures from costing forty round trips, and it is why the outcomes travel with the description rather than being
/// asked for afterwards.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_model"))]
#[tauri::command(rename = "open_model")]
pub async fn visuals_open_model(
  source: VisualSource,
  world: AssetWorldSpec,
  state: State<'_, VisualState>,
  assets: State<'_, AssetWorldState>,
) -> TauriResult<SelectedVisualDescription> {
  log::info!("Opening visual: {}", source.label());

  let package: VisualPackage = pack_source(&source)?;

  // The visual is its own first search step, so a texture beside the model wins over the same name in the project.
  let dependencies: VisualDependencies = assets.with_probe(&world, source.physical_path(), |probe| {
    VisualDependencies::resolve(&package.description, probe)
  })?;

  let description: SelectedVisualDescription = SelectedVisualDescription {
    source: source.clone(),
    description: package.description.clone(),
    dependencies: dependencies.clone(),
  };

  let mut selected: MutexGuard<Option<SelectedVisual>> = state
    .selected
    .lock()
    .map_err(|error| format!("Failed to open visual - selection state is unavailable: {error}"))?;

  *selected = Some(SelectedVisual {
    source,
    package,
    dependencies,
  });

  Ok(description)
}

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

  // Centred on the model unless the caller centred it elsewhere, and the effective world is what travels back: a texture
  // resolved through the model's own tree has to be readable through the same tree afterwards.
  let world: AssetWorldSpec = world.centred_on(source.physical_path());

  // Read and resolve inside one probe, so the model and its references are looked for in the same world: a second probe
  // could mount a source between the two calls and answer differently.
  let (package, dependencies): (VisualPackage, VisualDependencies) = assets.with_probe(&world, |probe| {
    let package: VisualPackage = pack_source(&source, probe)?;
    let dependencies: VisualDependencies = VisualDependencies::resolve(&package.description, probe);

    TauriResult::Ok((package, dependencies))
  })??;

  let description: SelectedVisualDescription = SelectedVisualDescription {
    source: source.clone(),
    world: world.clone(),
    description: package.description.clone(),
    dependencies: dependencies.clone(),
  };

  let mut selected: MutexGuard<Option<SelectedVisual>> = state
    .selected
    .lock()
    .map_err(|error| format!("Failed to open visual - selection state is unavailable: {error}"))?;

  *selected = Some(SelectedVisual {
    source,
    world,
    package,
    dependencies,
  });

  Ok(description)
}

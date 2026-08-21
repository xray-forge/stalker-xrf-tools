use std::path::PathBuf;
use std::sync::MutexGuard;

use tauri::State;
use xrf_visual::VisualPackage;

use crate::core::types::TauriResult;
use crate::plugins::visuals::read::pack_source;
use crate::plugins::visuals::state::{SelectedVisual, SelectedVisualDescription, VisualSource, VisualState};
use crate::plugins::visuals::textures::submesh_texture::SubmeshTexture;
use crate::plugins::visuals::textures::texture_resolver::VisualTextureResolver;

/// Select a visual and return what it contains.
///
/// Geometry is packed here and parked, so the `read_geometry` that follows serves the same parse rather
/// than repeating it. The bytes are not returned: a typed command cannot carry them, which is why they
/// are read separately.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_model"))]
#[tauri::command(rename = "open_model")]
pub async fn visuals_open_model(
  source: VisualSource,
  fallback_root: Option<String>,
  state: State<'_, VisualState>,
) -> TauriResult<SelectedVisualDescription> {
  log::info!("Opening visual: {}", source.label());

  let package: VisualPackage = pack_source(&source)?;
  let fallback_root: Option<PathBuf> = fallback_root.map(PathBuf::from);

  let textures: Vec<SubmeshTexture> = {
    let mut resolver: MutexGuard<VisualTextureResolver> = state
      .textures
      .lock()
      .map_err(|error| format!("Failed to open visual - texture resolver is unavailable: {error}"))?;

    resolver.resolve_submeshes(
      source.physical_path(),
      fallback_root.as_deref(),
      &package.description.submeshes,
    )
  };

  let description: SelectedVisualDescription = SelectedVisualDescription {
    source: source.clone(),
    description: package.description.clone(),
    textures: textures.clone(),
  };

  let mut selected: MutexGuard<Option<SelectedVisual>> = state
    .selected
    .lock()
    .map_err(|error| format!("Failed to open visual - selection state is unavailable: {error}"))?;

  *selected = Some(SelectedVisual {
    source,
    package,
    textures,
  });

  Ok(description)
}

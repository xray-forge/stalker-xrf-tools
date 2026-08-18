use std::fs;
use std::path::PathBuf;
use std::sync::MutexGuard;

use tauri::State;
use tauri::ipc::Response;
use xrf_visual::VisualPackage;

use crate::types::TauriResult;
use crate::visuals::read::pack_source;
use crate::visuals::state::{SelectedVisual, VisualSource, VisualState};
use crate::visuals::textures::submesh_texture::{SubmeshTexture, SubmeshTextureResolution};
use crate::visuals::textures::texture_resolver::VisualTextureResolver;

/// The DDS bytes behind one submesh's texture reference.
///
/// Addressed by source and reference for the reason geometry is: a user clicking through models faster than they load
/// must not be able to pair one model's texture with another's submesh. Serving the selected model reuses the resolution
/// `open_model` already did; any other source is resolved on the spot and changes no state.
///
/// Bytes pass through untouched. Decoding here would cost roughly four times the transfer and throw away the mip chain,
/// and the frontend's `DDSLoader` uploads the compressed blocks directly.
#[tauri::command(rename = "read_texture")]
pub async fn visuals_read_texture(
  source: VisualSource,
  reference: String,
  fallback_root: Option<String>,
  state: State<'_, VisualState>,
) -> TauriResult<Response> {
  log::info!("Reading visual texture '{reference}' for: {}", source.label());

  let parked: Option<SubmeshTextureResolution> = {
    let selected: MutexGuard<Option<SelectedVisual>> = state
      .selected
      .lock()
      .map_err(|error| format!("Failed to read texture - selection state is unavailable: {error}"))?;

    selected
      .as_ref()
      .filter(|current| current.source == source)
      .and_then(|current| {
        current
          .textures
          .iter()
          .find(|texture: &&SubmeshTexture| texture.reference.as_deref() == Some(reference.as_str()))
          .map(|texture| texture.resolution.clone())
      })
  };

  let resolution: SubmeshTextureResolution = match parked {
    Some(resolution) => resolution,
    None => resolve_afresh(&source, &reference, fallback_root, &state)?,
  };

  let Some(location) = resolution.location() else {
    return Err(format!("Texture '{reference}' resolves to no file"));
  };

  let path: PathBuf = location.absolute_path();
  let bytes: Vec<u8> =
    fs::read(&path).map_err(|error| format!("Failed to read texture '{}': {error}", path.display()))?;

  log::info!("Serving {} bytes of {}", bytes.len(), location.logical_path());

  Ok(Response::new(bytes))
}

/// Resolution for a source that is not the parked one, which is the same work `open_model` does.
///
/// Repacking the visual is the price of not trusting the caller's reference: a reference is only legitimate if the file
/// actually declares it, and the parse is what says so.
fn resolve_afresh(
  source: &VisualSource,
  reference: &str,
  fallback_root: Option<String>,
  state: &State<'_, VisualState>,
) -> TauriResult<SubmeshTextureResolution> {
  let package: VisualPackage = pack_source(source)?;

  if !package
    .description
    .submeshes
    .iter()
    .any(|submesh| submesh.texture_name.as_deref() == Some(reference))
  {
    return Err(format!("Visual '{}' declares no texture '{reference}'", source.label()));
  }

  let fallback_root: Option<PathBuf> = fallback_root.map(PathBuf::from);

  let order: Vec<PathBuf> = VisualTextureResolver::mount_order(source.physical_path(), fallback_root.as_deref());

  let mut resolver: MutexGuard<VisualTextureResolver> = state
    .textures
    .lock()
    .map_err(|error| format!("Failed to read texture - texture resolver is unavailable: {error}"))?;

  Ok(resolver.resolve(&order, reference))
}

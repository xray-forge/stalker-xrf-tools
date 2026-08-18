use std::path::PathBuf;
use std::sync::MutexGuard;

use tauri::State;
use tauri::ipc::Response;
use xrf_assets::XrayScope;
use xrf_visual::VisualPackage;

use crate::types::TauriResult;
use crate::visuals::read::pack_source;
use crate::visuals::state::{SelectedVisual, VisualSource, VisualState};
use crate::visuals::textures::texture_resolver::VisualTextureResolver;

/// Returns the untouched DDS bytes for a declared submesh texture.
///
/// The command validates the reference against the requested visual before reading through the VFS. VFS reads support
/// both loose and archived textures, and preserving the DDS payload lets `DDSLoader` upload its compressed mip chain.
#[tauri::command(rename = "read_texture")]
pub async fn visuals_read_texture(
  source: VisualSource,
  reference: String,
  fallback_root: Option<String>,
  state: State<'_, VisualState>,
) -> TauriResult<Response> {
  log::info!("Reading visual texture '{reference}' for: {}", source.label());

  if !declares_texture(&source, &reference, &state)? {
    return Err(format!("Visual '{}' declares no texture '{reference}'", source.label()));
  }

  let fallback_root: Option<PathBuf> = fallback_root.map(PathBuf::from);

  let mut resolver: MutexGuard<VisualTextureResolver> = state
    .textures
    .lock()
    .map_err(|error| format!("Failed to read texture - texture resolver is unavailable: {error}"))?;

  let scope: XrayScope = resolver.scope_for(source.physical_path(), fallback_root.as_deref());
  let bytes: Vec<u8> = resolver.read(&scope, &reference)?;

  log::info!("Serving {} bytes for '{reference}'", bytes.len());

  Ok(Response::new(bytes))
}

/// Checks whether a visual declares a texture reference.
///
/// Reuses the selected visual's parse when possible and otherwise parses the requested source. This prevents the texture
/// command from reading an unrelated asset by reference alone.
fn declares_texture(source: &VisualSource, reference: &str, state: &State<'_, VisualState>) -> TauriResult<bool> {
  let selected: MutexGuard<Option<SelectedVisual>> = state
    .selected
    .lock()
    .map_err(|error| format!("Failed to read texture - selection state is unavailable: {error}"))?;

  if let Some(current) = selected.as_ref().filter(|current| &current.source == source) {
    return Ok(
      current
        .textures
        .iter()
        .any(|texture| texture.reference.as_deref() == Some(reference)),
    );
  }

  drop(selected);

  let package: VisualPackage = pack_source(source)?;

  Ok(
    package
      .description
      .submeshes
      .iter()
      .any(|submesh| submesh.texture_name.as_deref() == Some(reference)),
  )
}

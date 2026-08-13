use std::sync::MutexGuard;

use serde::Serialize;
use tauri::State;
use xrf_archive::{ArchiveFileDescriptor, ArchiveProject};
use xrf_texture::dds_bytes_as_png;
use xrf_utils::encode_bytes_to_standard_base64;

use crate::archives::state::ArchiveProjectState;
use crate::types::TauriResult;

#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveImagePreview {
  pub name: String,
  pub width: u32,
  pub height: u32,
  /// PNG bytes, base64 encoded so the webview can use them directly as an image source.
  pub base64: String,
}

/// Decode an archived DDS into a PNG the webview can display.
///
/// Compressed entries are fine here, unlike the text preview: the bytes are decompressed on the way out
/// of the archive, so compression is invisible by the time there is an image to decode.
#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "read_image"))]
#[tauri::command(rename = "read_image")]
pub async fn archives_read_image(
  path: &str,
  state: State<'_, ArchiveProjectState>,
) -> TauriResult<ArchiveImagePreview> {
  log::info!("Reading archive image: {}", path);

  let lock: MutexGuard<Option<ArchiveProject>> = state
    .project
    .lock()
    .map_err(|error| format!("Failed to read image - archive state is unavailable: {error}"))?;

  let project: &ArchiveProject = lock
    .as_ref()
    .ok_or_else(|| String::from("Failed to read image - archive is not open"))?;

  let descriptor: &ArchiveFileDescriptor = project
    .files
    .get(path)
    .ok_or_else(|| format!("Failed to read image - '{path}' is not in the archive"))?;

  // Both rules come from the project policy, which is the same object the frontend decides with.
  if !project.read_policy.supports_image(path) {
    return Err(format!("Failed to read image - '{path}' is not a decodable texture"));
  }

  if descriptor.size_real > project.read_policy.maximum_image_size {
    return Err(format!(
      "Failed to read image - '{path}' exceeds the {} byte preview limit",
      project.read_policy.maximum_image_size
    ));
  }

  let bytes: Vec<u8> = project.read_file_bytes(path).map_err(|error| error.to_string())?;
  let (width, height, png) = dds_bytes_as_png(&bytes).map_err(|error| error.to_string())?;

  Ok(ArchiveImagePreview {
    name: descriptor.name.clone(),
    width,
    height,
    base64: encode_bytes_to_standard_base64(&png),
  })
}

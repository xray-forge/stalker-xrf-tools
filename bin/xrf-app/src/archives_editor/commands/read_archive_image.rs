use std::sync::MutexGuard;

use serde::Serialize;
use serde_json::{Value, json};
use tauri::State;
use xray_archive::{ArchiveFileDescriptor, ArchiveProject};
use xray_texture::dds_bytes_as_png;
use xray_utils::encode_bytes_to_base64;

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

/// Largest archived image this will decode, guarding against holding a very large texture in memory.
const MAXIMUM_IMAGE_SIZE: u32 = 32 * 1024 * 1024;

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
#[tauri::command]
pub async fn read_archive_image(path: &str, state: State<'_, ArchivesEditorState>) -> TauriResult<Value> {
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

  if descriptor.size_real > MAXIMUM_IMAGE_SIZE {
    return Err(format!(
      "Failed to read image - '{path}' exceeds the {MAXIMUM_IMAGE_SIZE} byte preview limit"
    ));
  }

  let bytes: Vec<u8> = project.read_file_bytes(path).map_err(|error| error.to_string())?;
  let (width, height, png) = dds_bytes_as_png(&bytes).map_err(|error| error.to_string())?;

  Ok(json!(ArchiveImagePreview {
    name: descriptor.name.clone(),
    width,
    height,
    base64: encode_bytes_to_base64(&png),
  }))
}

use std::sync::MutexGuard;

use serde::Serialize;
use tauri::State;
use xrf_archive::{ArchiveFileDescriptor, ArchiveProject};
use xrf_sound::{SoundFile, SoundMetadata};
use xrf_utils::encode_bytes_to_standard_base64;

use crate::archives_editor::state::ArchivesEditorState;
use crate::types::TauriResult;

/// The X-Ray source parameters carried in a sound's first vorbis comment.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveAudioParameters {
  pub min_distance: f32,
  pub max_distance: f32,
  pub base_volume: f32,
  pub game_type: u32,
  pub max_ai_distance: f32,
}

#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveAudioPreview {
  pub name: String,
  pub channels: u16,
  pub sample_rate: u32,
  /// Absent for a sound carrying no recognized X-Ray comment, where the engine uses its own defaults.
  pub parameters: Option<ArchiveAudioParameters>,
  /// The ogg bytes as stored, base64 encoded. The webview decodes vorbis itself.
  pub base64: String,
}

/// Hand an archived sound to the webview, along with whatever the engine would read from it.
#[cfg_attr(feature = "typescript-bindings", specta::specta)]
#[tauri::command]
pub async fn read_archive_audio(path: &str, state: State<'_, ArchivesEditorState>) -> TauriResult<ArchiveAudioPreview> {
  log::info!("Reading archive audio: {}", path);

  let lock: MutexGuard<Option<ArchiveProject>> = state
    .project
    .lock()
    .map_err(|error| format!("Failed to read audio - archive state is unavailable: {error}"))?;

  let project: &ArchiveProject = lock
    .as_ref()
    .ok_or_else(|| String::from("Failed to read audio - archive is not open"))?;

  let descriptor: &ArchiveFileDescriptor = project
    .files
    .get(path)
    .ok_or_else(|| format!("Failed to read audio - '{path}' is not in the archive"))?;

  if !project.read_policy.supports_audio(path) {
    return Err(format!("Failed to read audio - '{path}' is not a playable sound"));
  }

  if descriptor.size_real > project.read_policy.maximum_audio_size {
    return Err(format!(
      "Failed to read audio - '{path}' exceeds the {} byte preview limit",
      project.read_policy.maximum_audio_size
    ));
  }

  let bytes: Vec<u8> = project.read_file_bytes(path).map_err(|error| error.to_string())?;

  // A sound that cannot be parsed is still worth playing: plenty of ogg in a mod was not produced by
  // the x-ray tools, and refusing it would be refusing a file the webview handles perfectly well.
  let sound: Option<SoundFile> = match SoundFile::read_from_bytes(&bytes) {
    Ok(sound) => Some(sound),
    Err(error) => {
      log::warn!("Sound '{}' carries no readable x-ray headers: {}", path, error);

      None
    }
  };

  Ok(ArchiveAudioPreview {
    name: descriptor.name.clone(),
    channels: sound.as_ref().map_or(0, |it| it.channels),
    sample_rate: sound.as_ref().map_or(0, |it| it.sample_rate),
    parameters: sound.as_ref().and_then(|it| match &it.metadata {
      SoundMetadata::XRay { parameters, .. } => Some(ArchiveAudioParameters {
        min_distance: parameters.min_distance,
        max_distance: parameters.max_distance,
        base_volume: parameters.base_volume,
        game_type: parameters.game_type,
        max_ai_distance: parameters.max_ai_distance,
      }),
      SoundMetadata::EngineDefaults => None,
    }),
    base64: encode_bytes_to_standard_base64(&bytes),
  })
}

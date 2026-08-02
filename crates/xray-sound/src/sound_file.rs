use ogg::reading::PacketReader;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use symphonia::core::{
  codecs::audio::AudioDecoderOptions,
  formats::probe::Hint,
  formats::{FormatOptions, TrackType},
  io::MediaSourceStream,
  meta::MetadataOptions,
};
use symphonia::default::{get_codecs, get_probe};
use xray_error::{XRayError, XRayResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XRaySoundCommentVersion {
  V1,
  V2,
  V3,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XRaySoundParameters {
  pub min_distance: f32,
  pub max_distance: f32,
  pub base_volume: f32,
  pub game_type: u32,
  pub max_ai_distance: f32,
}

/// Metadata that controls how X-Ray configures a sound source.
#[derive(Clone, Debug, PartialEq)]
pub enum SoundMetadata {
  /// The engine uses its built-in sound-source defaults when no recognized X-Ray comment exists.
  EngineDefaults,
  XRay {
    version: XRaySoundCommentVersion,
    parameters: XRaySoundParameters,
  },
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoundFile {
  pub channels: u16,
  pub metadata: SoundMetadata,
  pub sample_rate: u32,
}

impl SoundFile {
  /// Read and fully decode an X-Ray Ogg/Vorbis sound file.
  ///
  /// Successful reads guarantee an Ogg/Vorbis stream that X-Ray can load. Sounds without a
  /// recognized X-Ray comment use the engine's default source parameters.
  pub fn read_from_path<P>(path: P) -> XRayResult<Self>
  where
    P: AsRef<Path>,
  {
    let path: &Path = path.as_ref();

    read_xray_sound(path).map_err(|error| {
      XRayError::new_verify_error(format!("Failed to read sound {}: {error}", path.display()))
    })
  }
}

fn read_xray_sound(path: &Path) -> Result<SoundFile, String> {
  let (channels, sample_rate, metadata): (u16, u32, SoundMetadata) =
    read_xray_sound_metadata(path)?;

  decode_vorbis_stream(path)?;

  Ok(SoundFile {
    channels,
    metadata,
    sample_rate,
  })
}

/// Read the binary X-Ray sound parameters stored in the first Vorbis comment.
fn read_xray_sound_metadata(path: &Path) -> Result<(u16, u32, SoundMetadata), String> {
  let file: File = File::open(path).map_err(|error| format!("Could not open sound: {error}"))?;
  let mut reader: PacketReader<File> = PacketReader::new(file);
  let identification_packet: Vec<u8> = read_expected_packet(&mut reader, "identification")?;
  let comment_packet: Vec<u8> = read_expected_packet(&mut reader, "comment")?;
  let (channels, sample_rate): (u16, u32) = parse_identification_packet(&identification_packet)?;

  if !comment_packet.starts_with(b"\x03vorbis") {
    return Err(String::from(
      "Ogg stream does not contain a Vorbis comment packet",
    ));
  }

  let metadata: SoundMetadata = first_vorbis_comment(&comment_packet[7..])?
    .map(|comment| parse_xray_comment(&comment))
    .transpose()?
    .flatten()
    .map(|(version, parameters)| SoundMetadata::XRay {
      version,
      parameters,
    })
    .unwrap_or(SoundMetadata::EngineDefaults);

  Ok((channels, sample_rate, metadata))
}

fn parse_identification_packet(packet: &[u8]) -> Result<(u16, u32), String> {
  if !packet.starts_with(b"\x01vorbis") {
    return Err(String::from(
      "Ogg stream does not contain a Vorbis identification packet",
    ));
  }

  let mut offset: usize = 7;
  let version: u32 = read_u32(packet, &mut offset, "Vorbis version")?;
  if version != 0 {
    return Err(format!("Unsupported Vorbis version: {version}"));
  }

  let channels: u16 = read_u8(packet, &mut offset, "channels")? as u16;
  let sample_rate: u32 = read_u32(packet, &mut offset, "sample rate")?;

  if channels == 0 || sample_rate == 0 {
    return Err(String::from(
      "Vorbis identification packet must define non-zero channels and sample rate",
    ));
  }

  Ok((channels, sample_rate))
}

fn read_expected_packet<R>(
  reader: &mut PacketReader<R>,
  packet_name: &str,
) -> Result<Vec<u8>, String>
where
  R: Read + Seek,
{
  reader
    .read_packet_expected()
    .map(|packet| packet.data)
    .map_err(|error| format!("Could not read Vorbis {packet_name} packet: {error}"))
}

fn first_vorbis_comment(packet: &[u8]) -> Result<Option<Vec<u8>>, String> {
  let mut offset: usize = 0;
  let vendor_length: usize = read_u32(packet, &mut offset, "vendor length")? as usize;
  read_bytes(packet, &mut offset, vendor_length, "vendor string")?;

  let comments_count: u32 = read_u32(packet, &mut offset, "comments count")?;
  if comments_count == 0 {
    return Ok(None);
  }

  let comment_length: usize = read_u32(packet, &mut offset, "first comment length")? as usize;
  Ok(Some(
    read_bytes(packet, &mut offset, comment_length, "first comment")?.to_vec(),
  ))
}

fn parse_xray_comment(
  comment: &[u8],
) -> Result<Option<(XRaySoundCommentVersion, XRaySoundParameters)>, String> {
  let mut offset: usize = 0;
  let version: u32 = read_u32(comment, &mut offset, "X-Ray comment version")?;
  let comment_version: XRaySoundCommentVersion = match version {
    0x0001 => XRaySoundCommentVersion::V1,
    0x0002 => XRaySoundCommentVersion::V2,
    0x0003 => XRaySoundCommentVersion::V3,
    _ => return Ok(None),
  };
  let min_distance: f32 = read_f32(comment, &mut offset, "minimum distance")?;
  let max_distance: f32 = read_f32(comment, &mut offset, "maximum distance")?;

  let (base_volume, game_type, max_ai_distance): (f32, u32, f32) = match comment_version {
    XRaySoundCommentVersion::V1 => {
      let game_type: u32 = read_u32(comment, &mut offset, "game type")?;
      (1.0, game_type, max_distance)
    }
    XRaySoundCommentVersion::V2 => {
      let base_volume: f32 = read_f32(comment, &mut offset, "base volume")?;
      let game_type: u32 = read_u32(comment, &mut offset, "game type")?;
      (base_volume, game_type, max_distance)
    }
    XRaySoundCommentVersion::V3 => {
      let base_volume: f32 = read_f32(comment, &mut offset, "base volume")?;
      let game_type: u32 = read_u32(comment, &mut offset, "game type")?;
      let max_ai_distance: f32 = read_f32(comment, &mut offset, "maximum AI distance")?;
      (base_volume, game_type, max_ai_distance)
    }
  };

  let parameters: XRaySoundParameters = XRaySoundParameters {
    min_distance,
    max_distance,
    base_volume,
    game_type,
    max_ai_distance,
  };

  if !parameters.min_distance.is_finite()
    || !parameters.max_distance.is_finite()
    || !parameters.base_volume.is_finite()
    || !parameters.max_ai_distance.is_finite()
  {
    return Err(String::from(
      "X-Ray sound parameters must be finite numbers",
    ));
  }

  if parameters.max_distance < 0.1 || parameters.max_ai_distance < 0.1 {
    return Err(String::from(
      "X-Ray sound maximum and maximum AI distances must be at least 0.1",
    ));
  }

  Ok(Some((comment_version, parameters)))
}

fn read_u8(bytes: &[u8], offset: &mut usize, field: &str) -> Result<u8, String> {
  let value: &[u8] = read_bytes(bytes, offset, 1, field)?;
  Ok(value[0])
}

fn read_u32(bytes: &[u8], offset: &mut usize, field: &str) -> Result<u32, String> {
  let value: &[u8] = read_bytes(bytes, offset, 4, field)?;
  Ok(u32::from_le_bytes(value.try_into().unwrap()))
}

fn read_f32(bytes: &[u8], offset: &mut usize, field: &str) -> Result<f32, String> {
  let value: &[u8] = read_bytes(bytes, offset, 4, field)?;
  Ok(f32::from_le_bytes(value.try_into().unwrap()))
}

fn read_bytes<'a>(
  bytes: &'a [u8],
  offset: &mut usize,
  length: usize,
  field: &str,
) -> Result<&'a [u8], String> {
  let end: usize = offset
    .checked_add(length)
    .ok_or_else(|| format!("Vorbis {field} length overflows the packet"))?;
  let value: &[u8] = bytes
    .get(*offset..end)
    .ok_or_else(|| format!("Vorbis packet ends before {field}"))?;
  *offset = end;
  Ok(value)
}

fn decode_vorbis_stream(path: &Path) -> Result<(), String> {
  let file: File = File::open(path).map_err(|error| format!("Could not open sound: {error}"))?;
  let stream = MediaSourceStream::new(Box::new(file), Default::default());
  let mut format = get_probe()
    .probe(
      &Hint::new(),
      stream,
      FormatOptions::default(),
      MetadataOptions::default(),
    )
    .map_err(|error| format!("Could not probe Ogg/Vorbis stream: {error}"))?;
  let track = format
    .default_track(TrackType::Audio)
    .ok_or_else(|| String::from("Ogg stream does not contain a default audio track"))?;
  let track_id: u32 = track.id;
  let codec_params = track
    .codec_params
    .as_ref()
    .and_then(|parameters| parameters.audio())
    .ok_or_else(|| String::from("Ogg stream does not contain an audio codec configuration"))?;
  let mut decoder = get_codecs()
    .make_audio_decoder(codec_params, &AudioDecoderOptions::default())
    .map_err(|error| format!("Could not initialize Vorbis decoder: {error}"))?;

  loop {
    let packet = match format.next_packet() {
      Ok(Some(packet)) => packet,
      Ok(None) => break,
      Err(error) => return Err(format!("Could not read Ogg/Vorbis packet: {error}")),
    };

    if packet.track_id != track_id {
      continue;
    }

    decoder
      .decode(&packet)
      .map_err(|error| format!("Could not decode Vorbis audio packet: {error}"))?;
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::{
    XRaySoundCommentVersion, XRaySoundParameters, first_vorbis_comment, parse_xray_comment,
  };

  fn xray_comment_v3(
    min_distance: f32,
    max_distance: f32,
    base_volume: f32,
    game_type: u32,
    max_ai_distance: f32,
  ) -> Vec<u8> {
    let mut comment: Vec<u8> = Vec::new();
    comment.extend(3u32.to_le_bytes());
    comment.extend(min_distance.to_le_bytes());
    comment.extend(max_distance.to_le_bytes());
    comment.extend(base_volume.to_le_bytes());
    comment.extend(game_type.to_le_bytes());
    comment.extend(max_ai_distance.to_le_bytes());
    comment
  }

  #[test]
  fn parses_valid_xray_v3_comment() {
    assert_eq!(
      parse_xray_comment(&xray_comment_v3(1.0, 50.0, 0.75, 7, 30.0)).unwrap(),
      Some((
        XRaySoundCommentVersion::V3,
        XRaySoundParameters {
          min_distance: 1.0,
          max_distance: 50.0,
          base_volume: 0.75,
          game_type: 7,
          max_ai_distance: 30.0,
        }
      ))
    );
  }

  #[test]
  fn rejects_xray_comment_with_invalid_distances() {
    let error: String = parse_xray_comment(&xray_comment_v3(1.0, 0.0, 1.0, 0, 0.0))
      .expect_err("Expected invalid sound distances to be rejected");

    assert_eq!(
      error,
      "X-Ray sound maximum and maximum AI distances must be at least 0.1"
    );
  }

  #[test]
  fn accepts_unknown_xray_comment_version_as_engine_defaults() {
    let mut comment: Vec<u8> = xray_comment_v3(1.0, 50.0, 1.0, 0, 50.0);
    comment[0..4].copy_from_slice(&4u32.to_le_bytes());

    assert_eq!(parse_xray_comment(&comment).unwrap(), None);
  }

  #[test]
  fn rejects_truncated_recognized_xray_comment() {
    let comment: Vec<u8> = 3u32.to_le_bytes().to_vec();

    assert_eq!(
      parse_xray_comment(&comment).unwrap_err(),
      "Vorbis packet ends before minimum distance"
    );
  }

  #[test]
  fn accepts_empty_vorbis_comments_as_engine_defaults() {
    let comment_packet: Vec<u8> = [0u32.to_le_bytes(), 0u32.to_le_bytes()].concat();

    assert_eq!(first_vorbis_comment(&comment_packet).unwrap(), None);
  }
}

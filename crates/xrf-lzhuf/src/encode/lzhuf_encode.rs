use xrf_error::{XrfError, XrfResult};

use crate::encode::lzhuf_encoder::LzhufEncoder;
use crate::lzhuf_constants::DECOMPRESSED_SIZE_HEADER_LENGTH;

/// Compress bytes into an X-Ray LZHUF blob, the exact byte layout `_compressLZ` produces.
///
/// The blob is a little-endian `u32` decompressed size followed by the bit stream, ready to be written as
/// the payload of a chunk whose id carries the compressed mark.
///
/// Output is not byte-identical to the engine's, and does not need to be: the engine also matches against
/// the pre-filled history window, which this does not. Both produce streams any X-Ray decoder accepts.
///
/// # Errors
///
/// Fails on empty input, which the engine's `Decode` refuses to read back, and on input too large for the
/// size header.
pub fn compress(source: &[u8]) -> XrfResult<Vec<u8>> {
  if source.is_empty() {
    return Err(XrfError::new_encoding_error(
      "LZHUF cannot code an empty source, which no X-Ray decoder accepts".to_string(),
    ));
  }

  let Ok(declared_size) = u32::try_from(source.len()) else {
    return Err(XrfError::new_encoding_error(format!(
      "LZHUF source of {} byte(s) exceeds the size the header can declare",
      source.len()
    )));
  };

  let stream: Vec<u8> = LzhufEncoder::new(source).encode()?;
  let mut blob: Vec<u8> = Vec::with_capacity(DECOMPRESSED_SIZE_HEADER_LENGTH + stream.len());

  blob.extend_from_slice(&declared_size.to_le_bytes());
  blob.extend_from_slice(&stream);

  Ok(blob)
}

#[cfg(test)]
mod tests {
  use super::compress;
  use crate::decode::lzhuf_decode::decompress;
  use crate::lzhuf_constants::{
    DECOMPRESSED_SIZE_HEADER_LENGTH, MATCH_LENGTH_MAX, REBUILD_FREQUENCY_LIMIT, RING_BUFFER_SIZE,
  };

  /// Deterministic bytes that barely compress, so each one costs its own symbol.
  fn pseudo_random_bytes(length: usize) -> Vec<u8> {
    let mut state: u32 = 0x9E37_79B1;

    (0..length)
      .map(|_| {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);

        (state >> 24) as u8
      })
      .collect()
  }

  fn assert_round_trips(source: &[u8]) -> Vec<u8> {
    let blob: Vec<u8> = compress(source).expect("source compresses");
    let restored: Vec<u8> = decompress(&blob).expect("blob decompresses");

    assert_eq!(restored, source, "round trip changed {} byte(s)", source.len());

    blob
  }

  #[test]
  fn round_trips_plain_text() {
    assert_round_trips(b"The Zone does not forgive mistakes, stalker.");
  }

  #[test]
  fn round_trips_a_single_byte() {
    assert_round_trips(&[0x42]);
  }

  #[test]
  fn round_trips_every_byte_value() {
    let source: Vec<u8> = (0..=u8::MAX).collect();

    assert_round_trips(&source);
  }

  #[test]
  fn round_trips_lengths_around_the_match_bounds() {
    // One byte either side of the shortest and longest codable match, where off-by-ones live.
    for length in [1, 2, 3, 4, MATCH_LENGTH_MAX - 1, MATCH_LENGTH_MAX, MATCH_LENGTH_MAX + 1] {
      assert_round_trips(&vec![b'#'; length]);
      assert_round_trips(&pseudo_random_bytes(length));
    }
  }

  #[test]
  fn round_trips_an_overlapping_run() {
    // One byte repeated far past the longest match, which only decodes if runs overlap correctly.
    assert_round_trips(&vec![b'z'; 10_000]);
  }

  #[test]
  fn round_trips_a_repeat_at_the_window_edge() {
    let mut source: Vec<u8> = b"a distinctive opening phrase".to_vec();
    let opening: Vec<u8> = source.clone();

    source.extend(pseudo_random_bytes(RING_BUFFER_SIZE - opening.len()));
    source.extend_from_slice(&opening);

    assert_round_trips(&source);
  }

  #[test]
  fn round_trips_data_long_enough_to_rebuild_the_tree() {
    // Incompressible bytes cost roughly one symbol each, so this crosses the rebuild limit twice.
    let source: Vec<u8> = pseudo_random_bytes(REBUILD_FREQUENCY_LIMIT as usize * 3);

    assert_round_trips(&source);
  }

  #[test]
  fn round_trips_repetitive_data_long_enough_to_rebuild_the_tree() {
    let phrase: &[u8] = b"stalker_ai_pack_variant_";
    let mut source: Vec<u8> = Vec::new();

    for index in 0..REBUILD_FREQUENCY_LIMIT as usize {
      source.extend_from_slice(phrase);
      source.extend_from_slice(index.to_string().as_bytes());
    }

    assert_round_trips(&source);
  }

  #[test]
  fn shrinks_repetitive_data() {
    let source: Vec<u8> = b"gamedata\\configs\\misc\\".repeat(500);
    let blob: Vec<u8> = assert_round_trips(&source);

    assert!(
      blob.len() * 10 < source.len(),
      "repetitive data should compress hard, got {} from {}",
      blob.len(),
      source.len()
    );
  }

  #[test]
  fn declares_the_decompressed_size_in_its_header() {
    let source: &[u8] = b"header check";
    let blob: Vec<u8> = compress(source).expect("source compresses");
    let header: [u8; DECOMPRESSED_SIZE_HEADER_LENGTH] = blob[..DECOMPRESSED_SIZE_HEADER_LENGTH]
      .try_into()
      .expect("header present");

    assert_eq!(u32::from_le_bytes(header) as usize, source.len());
  }

  #[test]
  fn refuses_an_empty_source() {
    assert!(compress(&[]).is_err());
  }
}

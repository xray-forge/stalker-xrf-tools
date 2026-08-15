use xrf_error::{XrfError, XrfResult};

use crate::decode::lzhuf_decoder::LzhufDecoder;
use crate::lzhuf_constants::{DECOMPRESSED_SIZE_HEADER_LENGTH, MATCH_LENGTH_MAX};

/// Most bytes one input byte may legitimately yield.
///
/// A match costs at least one bit of Huffman code plus nine bits of distance and yields at most
/// [`MATCH_LENGTH_MAX`] bytes, so ten bits can produce sixty bytes at the very best. Real streams stay far
/// under that; this only exists so a corrupt header cannot ask for a huge allocation. `Decode` in
/// `LzHuf.cpp` guards the same way, bounding the declared size by the size of the containing archive.
const MAX_EXPANSION_RATIO: usize = (MATCH_LENGTH_MAX * 8).div_ceil(10);

/// Decompress an X-Ray LZHUF blob, the exact byte layout `_decompressLZ` accepts.
///
/// The blob is a little-endian `u32` decompressed size followed by the bit stream. This is what X-Ray
/// stores in any chunk whose id carries the compressed mark, including archive file descriptors.
///
/// # Errors
///
/// Fails when the blob is too short to hold a header, declares a zero or implausible size, or ends before
/// the declared size has been produced.
pub fn decompress(source: &[u8]) -> XrfResult<Vec<u8>> {
  let Some((header, stream)) = source.split_at_checked(DECOMPRESSED_SIZE_HEADER_LENGTH) else {
    return Err(XrfError::new_parsing_error(format!(
      "LZHUF blob of {} byte(s) is too short to declare a decompressed size",
      source.len()
    )));
  };

  let declared_size: usize = match header.try_into() {
    Ok(bytes) => u32::from_le_bytes(bytes) as usize,
    Err(error) => {
      return Err(XrfError::new_parsing_error(format!(
        "LZHUF blob has no readable decompressed size: {error}"
      )));
    }
  };

  // `Decode` refuses a zero size rather than yielding nothing, because X-Ray never writes such a blob.
  if declared_size == 0 {
    return Err(XrfError::new_parsing_error(
      "LZHUF blob declares an empty decompressed size".to_string(),
    ));
  }

  let plausible_size: usize = stream.len().saturating_mul(MAX_EXPANSION_RATIO);

  if declared_size > plausible_size {
    return Err(XrfError::new_parsing_error(format!(
      "LZHUF blob declares {declared_size} byte(s) of output, more than {} byte(s) of stream can yield",
      stream.len()
    )));
  }

  let mut target: Vec<u8> = vec![0; declared_size];

  decompress_into(stream, &mut target)?;

  Ok(target)
}

/// Decompress a headerless X-Ray LZHUF stream into a buffer of known size.
///
/// Fills `target` completely. Use this when the decompressed size travels separately from the stream;
/// [`decompress`] is the entry point for a stream that carries its own size header.
///
/// # Errors
///
/// Fails when the stream ends before `target` is full, or when it decodes to a malformed coder state.
pub fn decompress_into(stream: &[u8], target: &mut [u8]) -> XrfResult<()> {
  LzhufDecoder::new(stream).decode_into(target)
}

#[cfg(test)]
mod tests {
  use super::{MAX_EXPANSION_RATIO, decompress};

  #[test]
  fn rejects_a_blob_without_a_size_header() {
    assert!(decompress(&[]).is_err());
    assert!(decompress(&[0x01, 0x02, 0x03]).is_err());
  }

  #[test]
  fn rejects_a_blob_declaring_nothing() {
    assert!(decompress(&[0, 0, 0, 0, 0xff]).is_err());
  }

  #[test]
  fn rejects_a_size_no_stream_that_short_could_produce() {
    // Four bytes of stream cannot yield 16 MB, however the bits decode.
    let mut blob: Vec<u8> = 0x0100_0000u32.to_le_bytes().to_vec();

    blob.extend_from_slice(&[0xff; 4]);

    assert!(decompress(&blob).is_err());
  }

  #[test]
  fn bounds_expansion_above_the_best_possible_match() {
    // Sixty bytes for ten bits is the theoretical best; the guard must not sit below it.
    const { assert!(MAX_EXPANSION_RATIO >= 48) };
  }
}

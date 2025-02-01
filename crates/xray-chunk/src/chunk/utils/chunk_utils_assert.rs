use crate::chunk::reader::chunk_reader::ChunkReader;
use xray_error::{XRayError, XRayResult};

/// Assert chunk ended and has no remaining data or fail with error.
#[inline(always)]
pub fn assert_chunk_read(chunk: &ChunkReader, message: &str) -> XRayResult {
  if chunk.is_ended() {
    Ok(())
  } else {
    Err(XRayError::new_chunk_not_ended_error(
      message,
      chunk.read_bytes_remain(),
    ))
  }
}

/// Assert chunk vector read with correct len.
#[inline(always)]
pub fn assert_chunk_vector_read<T>(data: &[T], expected: usize, message: &str) -> XRayResult {
  if data.len() == expected {
    Ok(())
  } else {
    Err(XRayError::new_invalid_error(message))
  }
}

use crate::chunk::reader::chunk_reader::ChunkReader;
use crate::{ChunkError, ChunkResult};

/// Assert chunk ended and has no remaining data or fail with error.
pub fn assert_chunk_read(chunk: &ChunkReader, message: &str) -> ChunkResult<()> {
  if chunk.is_ended() {
    Ok(())
  } else {
    Err(ChunkError::new_not_ended_chunk_error(
      message,
      chunk.read_bytes_remain(),
    ))
  }
}

/// Assert chunk vector read with correct len.
pub fn assert_chunk_vector_read<T>(data: &[T], expected: usize, message: &str) -> ChunkResult<()> {
  if data.len() == expected {
    Ok(())
  } else {
    Err(ChunkError::new_invalid_chunk_error(message))
  }
}

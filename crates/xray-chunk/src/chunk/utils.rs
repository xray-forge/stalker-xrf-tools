use crate::chunk::reader::ChunkReader;
use crate::{ChunkError, ChunkResult};
use byteorder::{ByteOrder, ReadBytesExt};
use std::io::Read;

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

/// Find chink in list by id.
pub fn find_optional_chunk_by_id(chunks: &[ChunkReader], id: u32) -> Option<ChunkReader> {
  chunks.iter().find(|it| it.id == id).cloned()
}

/// Find chink in list by id.
pub fn find_one_of_optional_chunk_by_id(
  chunks: &[ChunkReader],
  ids: &[u32],
) -> Option<(u32, ChunkReader)> {
  for id in ids {
    if let Some(chunk) = chunks.iter().find(|it| it.id == *id).cloned() {
      return Some((*id, chunk));
    }
  }

  None
}

/// Find required chunk in list by id.
pub fn find_required_chunk_by_id(chunks: &[ChunkReader], id: u32) -> ChunkResult<ChunkReader> {
  match chunks.iter().find(|it| it.id == id).cloned() {
    None => Err(ChunkError::new_not_found_chunk_error(format!(
      "Chunk with ID {id} was not found"
    ))),
    Some(it) => Ok(it),
  }
}

/// Find required chunk in list by one of ids.
pub fn find_one_of_required_chunks_by_id(
  chunks: &[ChunkReader],
  ids: &[u32],
) -> ChunkResult<(u32, ChunkReader)> {
  for id in ids {
    if let Some(chunk) = chunks.iter().find(|it| it.id == *id).cloned() {
      return Ok((*id, chunk));
    }
  }

  Err(ChunkError::new_not_found_chunk_error(format!(
    "Chunk with one of IDs {ids:?} was not found"
  )))
}

/// Read chunk as u16 value, verify remaining data is 0.
pub fn read_u16_chunk<T: ByteOrder>(reader: &mut ChunkReader) -> ChunkResult<u16> {
  let data: u16 = reader.read_u16::<T>()?;

  assert!(
    reader.is_ended(),
    "Expect u16 chunk to be ended, {} remain",
    reader.read_bytes_remain()
  );

  Ok(data)
}

/// Read chunk as u32 value, verify remaining data is 0.
pub fn read_u32_chunk<T: ByteOrder>(reader: &mut ChunkReader) -> ChunkResult<u32> {
  let data: u32 = reader.read_u32::<T>()?;

  assert!(
    reader.is_ended(),
    "Expect u32 chunk to be ended, {} remain",
    reader.read_bytes_remain()
  );

  Ok(data)
}

/// Read chunk as f32 value, verify remaining data is 0.
pub fn read_f32_chunk<T: ByteOrder>(reader: &mut ChunkReader) -> ChunkResult<f32> {
  let data: f32 = reader.read_f32::<T>()?;

  assert!(
    reader.is_ended(),
    "Expect f32 chunk to be ended, {} remain",
    reader.read_bytes_remain()
  );

  Ok(data)
}

/// Read chunk as f32 vector value, verify remaining data is 0.
pub fn read_f32_vector_chunk<T: ByteOrder>(
  reader: &mut ChunkReader,
) -> ChunkResult<(f32, f32, f32)> {
  let data: (f32, f32, f32) = (
    reader.read_f32::<T>()?,
    reader.read_f32::<T>()?,
    reader.read_f32::<T>()?,
  );

  assert!(
    reader.is_ended(),
    "Expect f32 vector chunk to be ended, {} remain",
    reader.read_bytes_remain()
  );
  Ok(data)
}

/// Read chunk as binary data till reader end, verify remaining data is 0.
pub fn read_till_end_binary_chunk(reader: &mut ChunkReader) -> ChunkResult<Vec<u8>> {
  let mut data: Vec<u8> = vec![0; reader.read_bytes_remain() as usize];

  reader.read_exact(&mut data)?;

  assert!(
    reader.is_ended(),
    "Expect binary data chunk to be ended, {} remain",
    reader.read_bytes_remain()
  );

  Ok(data)
}

/// Read chunk as containing string, verify remaining data is 0.
pub fn read_null_terminated_win_string_chunk(reader: &mut ChunkReader) -> ChunkResult<String> {
  let data: String = reader.read_null_terminated_win_string()?;

  assert!(
    reader.is_ended(),
    "Expect string chunk to be ended, {} remain",
    reader.read_bytes_remain()
  );

  Ok(data)
}

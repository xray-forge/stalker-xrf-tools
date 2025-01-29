use crate::{ChunkError, ChunkReader, ChunkResult};

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

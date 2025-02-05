use crate::reader::chunk_reader::ChunkReader;
use xray_error::{XRayError, XRayResult};

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
pub fn find_required_chunk_by_id(chunks: &[ChunkReader], id: u32) -> XRayResult<ChunkReader> {
  match chunks.iter().find(|it| it.id == id).cloned() {
    None => Err(XRayError::new_not_found_error(format!(
      "Chunk with ID {} was not found", id
    ))),
    Some(it) => Ok(it),
  }
}

/// Find required chunk in list by one of ids.
pub fn find_one_of_required_chunks_by_id(
  chunks: &[ChunkReader],
  ids: &[u32],
) -> XRayResult<(u32, ChunkReader)> {
  for id in ids {
    if let Some(chunk) = chunks.iter().find(|it| it.id == *id).cloned() {
      return Ok((*id, chunk));
    }
  }

  Err(XRayError::new_not_found_error(format!(
    "Chunk with one of IDs {} was not found",
    ids.iter().map(u32::to_string).collect::<Vec<_>>().join(",")
  )))
}

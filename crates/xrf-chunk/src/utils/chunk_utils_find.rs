use xrf_error::{XrfError, XrfResult};

use crate::reader::chunk_reader::ChunkReader;
use crate::source::chunk_data_source::ChunkDataSource;

/// Find chunk in list by id.
///
/// Generic over the data source so a chunked format can be read from bytes as well as from a file. An archived entry has no
/// file to slice, so without this nothing chunked could be read out of a volume.
#[inline]
pub fn find_optional_chunk_by_id<S: ChunkDataSource>(chunks: &[ChunkReader<S>], id: u32) -> Option<ChunkReader<S>> {
  chunks.iter().find(|it| it.id == id).cloned()
}

/// Find chunk in list by id.
#[inline]
pub fn find_one_of_optional_chunk_by_id<S: ChunkDataSource>(
  chunks: &[ChunkReader<S>],
  ids: &[u32],
) -> Option<(u32, ChunkReader<S>)> {
  for id in ids {
    if let Some(chunk) = chunks.iter().find(|it| it.id == *id).cloned() {
      return Some((*id, chunk));
    }
  }

  None
}

/// Find required chunk in list by id.
#[inline]
pub fn find_required_chunk_by_id<S: ChunkDataSource>(chunks: &[ChunkReader<S>], id: u32) -> XrfResult<ChunkReader<S>> {
  match chunks.iter().find(|it| it.id == id).cloned() {
    None => Err(XrfError::new_not_found_error(format!(
      "Chunk with ID {} was not found",
      id
    ))),
    Some(it) => Ok(it),
  }
}

/// Find required chunk in list by one of ids.
#[inline]
pub fn find_one_of_required_chunks_by_id<S: ChunkDataSource>(
  chunks: &[ChunkReader<S>],
  ids: &[u32],
) -> XrfResult<(u32, ChunkReader<S>)> {
  for id in ids {
    if let Some(chunk) = chunks.iter().find(|it| it.id == *id).cloned() {
      return Ok((*id, chunk));
    }
  }

  Err(XrfError::new_not_found_error(format!(
    "Chunk with one of IDs {} was not found",
    ids.iter().map(u32::to_string).collect::<Vec<_>>().join(",")
  )))
}

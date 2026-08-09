use std::fs::File;
use std::path::Path;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, find_optional_chunk_by_id, find_required_chunk_by_id};
use xray_error::{XRayError, XRayResult};

use crate::level::level_header_chunk::LevelHeaderChunk;
use crate::level::level_shaders_chunk::LevelShadersChunk;

/// Descriptor of the compiled `level` file used by xray game engine.
///
/// Only the chunks needed to validate a built level bundle are read. Geometry, portals, sectors,
/// glows and lights are left untouched - the engine streams them lazily and they are orders of
/// magnitude larger than the rest of the file.
///
/// Root level chunks by ID:
/// 1 - header
/// 2 - shaders
/// 3 - visuals
/// 4 - portals
/// 6 - dynamic light
/// 7 - glows
/// 8 - sectors
/// 9 - vertex buffer
/// 10 - index buffer
/// 11 - slide window items
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelFile {
  pub header: LevelHeaderChunk,
  /// Absent shaders chunk is a fatal defect rather than a read failure, so it is reported as data
  /// instead of an error. The renderer asserts on it with `Level doesn't builded correctly.` -
  /// quoted verbatim from `r2_loader.cpp` so it matches what the engine prints to the log.
  pub shaders: Option<LevelShadersChunk>,
}

impl LevelFile {
  /// Read level file from provided path.
  pub fn read_from_path<T: ByteOrder, P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    Self::read_from_file::<T>(File::open(path).map_err(|error| {
      XRayError::new_not_found_error(format!(
        "Level file was not read: {}, error: {}",
        path.as_ref().display(),
        error
      ))
    })?)
  }

  /// Read level file from file.
  pub fn read_from_file<T: ByteOrder>(file: File) -> XRayResult<Self> {
    let chunks: Vec<ChunkReader> = ChunkReader::from_file(file)?.read_children()?;

    Self::read_from_chunks::<T>(&chunks)
  }

  /// Read level file from chunks.
  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> XRayResult<Self> {
    Ok(Self {
      header: find_required_chunk_by_id(chunks, LevelHeaderChunk::CHUNK_ID)?.read_xr::<T, _>()?,
      shaders: match find_optional_chunk_by_id(chunks, LevelShadersChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
    })
  }
}

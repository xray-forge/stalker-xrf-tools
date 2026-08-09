use std::fs::File;
use std::path::Path;

use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, find_optional_chunk_by_id};
use xray_error::{XRayError, XRayResult};

use crate::thm::chunks::thm_bump_chunk::ThmBumpChunk;

/// Texture descriptor file, `STextureParams` in the engine (`ETextureParams.cpp`).
///
/// Reads only the parts the toolchain needs. A thm carries several chunks of authoring metadata,
/// but the one with runtime consequences is the bump declaration, so everything else stays
/// unparsed and any edit patches raw chunks rather than re-serializing, see
/// [`crate::ThmBumpProcessor`].
#[derive(Debug, Serialize, Deserialize)]
pub struct ThmFile {
  pub bump: Option<ThmBumpChunk>,
}

impl ThmFile {
  pub fn read_from_path<T: ByteOrder, P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    Self::read_from_file::<T>(File::open(path).map_err(|error| {
      XRayError::new_not_found_error(format!(
        "THM file was not read: {}, error: {}",
        path.as_ref().display(),
        error
      ))
    })?)
  }

  pub fn read_from_file<T: ByteOrder>(file: File) -> XRayResult<Self> {
    let chunks: Vec<ChunkReader> = ChunkReader::from_file(file)?.read_children()?;

    Ok(Self {
      bump: match find_optional_chunk_by_id(&chunks, ThmBumpChunk::CHUNK_ID) {
        Some(mut chunk) => Some(chunk.read_xr::<T, ThmBumpChunk>()?),
        None => None,
      },
    })
  }

  /// Bump texture this descriptor asks the engine to resolve, if any.
  pub fn used_bump_name(&self) -> Option<&str> {
    self
      .bump
      .as_ref()
      .filter(|bump| bump.is_used())
      .map(|bump| bump.name.as_str())
  }
}

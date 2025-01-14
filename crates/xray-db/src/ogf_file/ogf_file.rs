use crate::ogf_file::chunks::ogf_description_chunk::OgfDescriptionChunk;
use crate::ogf_file::chunks::ogf_header_chunk::OgfHeaderChunk;
use crate::ogf_file::chunks::ogf_kinematics_chunk::OgfKinematicsChunk;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use fileslice::FileSlice;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use xray_chunk::{
  find_one_of_optional_chunk_by_id, find_one_of_required_chunks_by_id, find_optional_chunk_by_id,
  find_required_chunk_by_id, ChunkReader,
};

/// FMesh in c++ codebase.
#[derive(Debug, Serialize, Deserialize)]
pub struct OgfFile {
  pub header: OgfHeaderChunk,
  pub description: Option<OgfDescriptionChunk>,
  pub kinematics: Option<OgfKinematicsChunk>,
}

impl OgfFile {
  pub fn read_from_path<T: ByteOrder>(path: &Path) -> DatabaseResult<Self> {
    log::info!("Reading ogf path: {:?}", path);

    Self::read_from_file::<T>(File::open(path)?)
  }

  pub fn read_from_file<T: ByteOrder>(file: File) -> DatabaseResult<Self> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = reader.read_children();

    log::info!(
      "Reading ogf file, {} chunks, {} bytes",
      chunks.len(),
      reader.read_bytes_len(),
    );

    for chunk in &chunks {
      log::info!(
        "Ogf chunk: {} ({:#x}) - {} bytes",
        chunk.id,
        chunk.id,
        chunk.size
      );
    }

    Self::read_from_chunks::<T>(&chunks)
  }

  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> DatabaseResult<Self> {
    Ok(Self {
      header: OgfHeaderChunk::read::<T>(&mut find_required_chunk_by_id(
        chunks,
        OgfHeaderChunk::CHUNK_ID,
      )?)?,
      description: match find_optional_chunk_by_id(chunks, OgfDescriptionChunk::CHUNK_ID) {
        Some(mut it) => Some(OgfDescriptionChunk::read::<T>(&mut it)?),
        None => None,
      },
      kinematics: match find_one_of_optional_chunk_by_id(
        chunks,
        &[
          OgfKinematicsChunk::CHUNK_ID,
          OgfKinematicsChunk::CHUNK_ID_OLD,
        ],
      ) {
        Some((id, mut it)) => Some(OgfKinematicsChunk::read::<T>(&mut it, id)?),
        None => None,
      },
    })
  }

  /// Read only list of motion refs specifically and skip other data parts.
  pub fn read_motion_refs_from_path<T: ByteOrder>(path: &Path) -> DatabaseResult<Vec<String>> {
    Self::read_motions_refs_from_file::<T>(File::open(path)?)
  }

  /// Read only list of motion refs specifically and skip other data parts.
  pub fn read_motions_refs_from_file<T: ByteOrder>(file: File) -> DatabaseResult<Vec<String>> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = reader.read_children();

    log::info!(
      "Reading ogf file motion refs, {} chunks, {} bytes",
      chunks.len(),
      reader.read_bytes_len(),
    );

    let (chunk_id, mut chunk) = find_one_of_required_chunks_by_id(
      &chunks,
      &[
        OgfKinematicsChunk::CHUNK_ID,
        OgfKinematicsChunk::CHUNK_ID_OLD,
      ],
    )?;

    Ok(OgfKinematicsChunk::read::<T>(&mut chunk, chunk_id)?.motion_refs)
  }
}

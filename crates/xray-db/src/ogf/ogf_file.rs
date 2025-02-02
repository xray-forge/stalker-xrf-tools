use crate::ogf::chunks::ogf_bones_chunk::OgfBonesChunk;
use crate::ogf::chunks::ogf_children_chunk::OgfChildrenChunk;
use crate::ogf::chunks::ogf_description_chunk::OgfDescriptionChunk;
use crate::ogf::chunks::ogf_header_chunk::OgfHeaderChunk;
use crate::ogf::chunks::ogf_kinematics_chunk::OgfKinematicsChunk;
use crate::ogf::chunks::ogf_texture_chunk::OgfTextureChunk;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use xray_chunk::{
  find_one_of_optional_chunk_by_id, find_one_of_required_chunks_by_id, find_optional_chunk_by_id,
  find_required_chunk_by_id, ChunkReader,
};
use xray_error::{XRayError, XRayResult};

/// FMesh in c++ codebase.
#[derive(Debug, Serialize, Deserialize)]
pub struct OgfFile {
  pub header: OgfHeaderChunk,
  pub texture: Option<OgfTextureChunk>,
  pub bones: Option<OgfBonesChunk>,
  pub children: Option<OgfChildrenChunk>,
  pub description: Option<OgfDescriptionChunk>,
  pub kinematics: Option<OgfKinematicsChunk>,
}

impl OgfFile {
  pub fn read_from_path<T: ByteOrder, D: AsRef<Path>>(path: D) -> XRayResult<Self> {
    log::info!("Reading ogf path: {}", path.as_ref().display());

    Self::read_from_file::<T>(File::open(&path).map_err(|error| {
      XRayError::new_not_found_error(format!(
        "OGF file was not read: {}, error: {}",
        path.as_ref().display(),
        error
      ))
    })?)
  }

  pub fn read_from_file<T: ByteOrder>(file: File) -> XRayResult<Self> {
    Self::read_from_chunk::<T>(&mut ChunkReader::from_file(file)?)
  }

  pub fn read_from_chunk<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
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

  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> XRayResult<Self> {
    Ok(Self {
      header: OgfHeaderChunk::read::<T>(&mut find_required_chunk_by_id(
        chunks,
        OgfHeaderChunk::CHUNK_ID,
      )?)?,
      texture: match find_optional_chunk_by_id(chunks, OgfTextureChunk::CHUNK_ID) {
        Some(mut it) => Some(OgfTextureChunk::read::<T>(&mut it)?),
        None => None,
      },
      bones: match find_optional_chunk_by_id(chunks, OgfBonesChunk::CHUNK_ID) {
        Some(mut it) => Some(OgfBonesChunk::read::<T>(&mut it)?),
        None => None,
      },
      children: match find_optional_chunk_by_id(chunks, OgfChildrenChunk::CHUNK_ID) {
        Some(mut it) => Some(OgfChildrenChunk::read::<T>(&mut it)?),
        None => None,
      },
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
  pub fn read_motion_refs_from_path<T: ByteOrder, P: AsRef<Path>>(
    path: &P,
  ) -> XRayResult<Vec<String>> {
    Self::read_motions_refs_from_file::<T>(File::open(path)?)
  }

  /// Read only list of motion refs specifically and skip other data parts.
  pub fn read_motions_refs_from_file<T: ByteOrder>(file: File) -> XRayResult<Vec<String>> {
    let mut reader: ChunkReader = ChunkReader::from_file(file)?;
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

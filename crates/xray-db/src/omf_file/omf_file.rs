use crate::omf_file::chunks::omf_motions_chunk::OmfMotionsChunk;
use crate::omf_file::chunks::omf_parameters_chunk::OmfParametersChunk;
use crate::{DatabaseError, DatabaseResult};
use byteorder::ByteOrder;
use fileslice::FileSlice;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use xray_chunk::{find_required_chunk_by_id, ChunkReader};

// c++ CKinematicsAnimated
#[derive(Debug, Serialize, Deserialize)]
pub struct OmfFile {
  pub parameters: OmfParametersChunk,
  pub motions: OmfMotionsChunk,
}

impl OmfFile {
  pub const SUPPORTED_VERSIONS: [u16; 2] = [3, 4];

  pub fn read_from_path<T: ByteOrder, D: AsRef<Path>>(path: D) -> DatabaseResult<Self> {
    Self::read_from_file::<T>(File::open(&path).map_err(|error| {
      DatabaseError::new_not_found_error(format!(
        "OMF file was not read: {:?}, error: {error}",
        path.as_ref(),
      ))
    })?)
  }

  pub fn read_from_file<T: ByteOrder>(file: File) -> DatabaseResult<Self> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = reader.read_children();

    log::info!(
      "Reading omf file, {} chunks, {} bytes",
      chunks.len(),
      reader.read_bytes_len(),
    );

    for chunk in &chunks {
      log::info!(
        "Omf chunk: {} ({:#x}) - {} bytes",
        chunk.id,
        chunk.id,
        chunk.size
      );
    }

    Self::read_from_chunks::<T>(&chunks)
  }

  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> DatabaseResult<Self> {
    assert_eq!(
      chunks.len(),
      2,
      "Unexpected chunks count in omf file, expected 2"
    );

    let parameters: OmfParametersChunk = OmfParametersChunk::read::<T>(
      &mut find_required_chunk_by_id(chunks, OmfParametersChunk::CHUNK_ID)?,
    )
    .map_err(|error| {
      DatabaseError::new_read_error(format!("Failed to read OMF parameters: {error}"))
    })?;

    let motions: OmfMotionsChunk = OmfMotionsChunk::read::<T>(&mut find_required_chunk_by_id(
      chunks,
      OmfMotionsChunk::CHUNK_ID,
    )?)
    .map_err(|error| {
      DatabaseError::new_read_error(format!("Failed to read OMF motions: {error}"))
    })?;

    if parameters.motions.len() != motions.motions.len() {
      return Err(DatabaseError::new_parse_error(format!(
        "Unexpected data stored in OMF file, count of motions and motions definitions mismatch: {} got, {} expected",
        parameters.motions.len(),
        motions.motions.len()
      )));
    }

    Ok(Self {
      parameters,
      motions,
    })
  }
}

impl OmfFile {
  /// Read only list of motions specifically and skip other data parts.
  pub fn read_motions_from_path<T: ByteOrder>(path: &Path) -> DatabaseResult<Vec<String>> {
    Self::read_motions_from_file::<T>(File::open(path)?)
  }

  pub fn read_motions_from_file<T: ByteOrder>(file: File) -> DatabaseResult<Vec<String>> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = reader.read_children();

    log::info!(
      "Reading omf file motions, {} chunks, {} bytes",
      chunks.len(),
      reader.read_bytes_len(),
    );

    Ok(
      OmfMotionsChunk::read::<T>(&mut find_required_chunk_by_id(
        &chunks,
        OmfMotionsChunk::CHUNK_ID,
      )?)?
      .motions
      .iter()
      .map(|it| it.name.clone())
      .collect(),
    )
  }
}

impl OmfFile {
  pub fn get_bones(&self) -> Vec<&str> {
    self
      .parameters
      .parts
      .iter()
      .flat_map(|it| it.get_bones())
      .collect::<Vec<_>>()
  }

  pub fn get_bones_count(&self) -> usize {
    self
      .parameters
      .parts
      .iter()
      .map(|it| it.get_bones().len())
      .sum::<usize>()
  }
}

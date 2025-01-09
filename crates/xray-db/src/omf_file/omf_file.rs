use crate::omf_file::chunks::omf_motions_chunk::OmfMotionsChunk;
use crate::omf_file::chunks::omf_parameters_chunk::OmfParametersChunk;
use crate::{DatabaseParseError, DatabaseResult};
use byteorder::ByteOrder;
use fileslice::FileSlice;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use xray_chunk::{find_chunk_by_id, ChunkReader};

// c++ CKinematicsAnimated
#[derive(Debug, Serialize, Deserialize)]
pub struct OmfFile {
  pub parameters: OmfParametersChunk,
  pub motions: OmfMotionsChunk,
}

impl OmfFile {
  pub fn read_from_path<T: ByteOrder>(path: &Path) -> DatabaseResult<Self> {
    Self::read_from_file::<T>(File::open(path)?)
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
      &mut find_chunk_by_id(chunks, OmfParametersChunk::CHUNK_ID)
        .expect("OMF parameters chunk not found"),
    )?;

    let motions: OmfMotionsChunk = OmfMotionsChunk::read::<T>(
      &mut find_chunk_by_id(chunks, OmfMotionsChunk::CHUNK_ID)
        .expect("OMF motions chunk not found"),
    )?;

    if parameters.motions.len() != motions.motions.len() {
      return Err(DatabaseParseError::new_database_error(format!(
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
use crate::omf::chunks::omf_motions_chunk::OmfMotionsChunk;
use crate::omf::chunks::omf_parameters_chunk::OmfParametersChunk;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use xray_chunk::{find_required_chunk_by_id, ChunkReader};
use xray_error::{XRayError, XRayResult};

// c++ CKinematicsAnimated
#[derive(Debug, Serialize, Deserialize)]
pub struct OmfFile {
  pub parameters: OmfParametersChunk,
  pub motions: OmfMotionsChunk,
}

impl OmfFile {
  pub const SUPPORTED_VERSIONS: [u16; 2] = [3, 4];

  pub fn read_from_path<T: ByteOrder, P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    Self::read_from_file::<T>(File::open(path).map_err(|error| {
      XRayError::new_not_found_error(format!(
        "OMF file was not read: {}, error: {}",
        path.as_ref().display(),
        error
      ))
    })?)
  }

  pub fn read_from_file<T: ByteOrder>(file: File) -> XRayResult<Self> {
    Self::read_from_chunks::<T>(&ChunkReader::from_file(file)?.read_children())
  }

  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> XRayResult<Self> {
    assert_eq!(
      chunks.len(),
      2,
      "Unexpected chunks count in omf file, expected 2"
    );

    let parameters: OmfParametersChunk =
      find_required_chunk_by_id(chunks, OmfParametersChunk::CHUNK_ID)?
        .read_xr::<T, _>()
        .map_err(|error| {
          XRayError::new_read_error(format!("Failed to read OMF parameters: {error}"))
        })?;

    let motions: OmfMotionsChunk = find_required_chunk_by_id(chunks, OmfMotionsChunk::CHUNK_ID)?
      .read_xr::<T, _>()
      .map_err(|error| XRayError::new_read_error(format!("Failed to read OMF motions: {error}")))?;

    if parameters.motions.len() != motions.motions.len() {
      return Err(XRayError::new_parsing_error(format!(
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
  pub fn read_motions_from_path<T: ByteOrder, P: AsRef<Path>>(path: P) -> XRayResult<Vec<String>> {
    Self::read_motions_from_file::<T>(File::open(path)?)
  }

  pub fn read_motions_from_file<T: ByteOrder>(file: File) -> XRayResult<Vec<String>> {
    let mut reader: ChunkReader = ChunkReader::from_file(file)?;
    let chunks: Vec<ChunkReader> = reader.read_children();

    log::info!(
      "Reading omf file motions, {} chunks, {} bytes",
      chunks.len(),
      reader.read_bytes_len(),
    );

    Ok(
      find_required_chunk_by_id(&chunks, OmfMotionsChunk::CHUNK_ID)?
        .read_xr::<T, OmfMotionsChunk>()?
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

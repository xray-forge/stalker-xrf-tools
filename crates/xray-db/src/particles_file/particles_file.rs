use crate::chunk::reader::ChunkReader;
use crate::particles_file::particles_effects_chunk::ParticlesEffectsChunk;
use crate::particles_file::particles_firstgen_chunk::ParticlesFirstgenChunk;
use crate::particles_file::particles_groups_chunk::ParticlesGroupsChunk;
use crate::particles_file::particles_version_chunk::ParticlesVersionChunk;
use byteorder::ByteOrder;
use fileslice::FileSlice;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesFile {
  pub version: ParticlesVersionChunk,
  pub effects: ParticlesEffectsChunk,
  pub groups: ParticlesGroupsChunk,
}

impl ParticlesFile {
  /// Read particles xr file from provided path.
  pub fn read_from_path<T: ByteOrder>(path: &Path) -> io::Result<ParticlesFile> {
    Self::read_from_file::<T>(File::open(path)?)
  }

  /// Read particles xr from file.
  pub fn read_from_file<T: ByteOrder>(file: File) -> io::Result<ParticlesFile> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);
    let chunk_ids: Vec<u32> = chunks.iter().map(|it| it.index).collect();

    log::info!(
      "Parsed particles file meta, {} chunks, {} bytes, {:?} chunks",
      chunks.len(),
      reader.read_bytes_len(),
      chunk_ids
    );

    assert!(
      !chunk_ids.contains(&ParticlesFirstgenChunk::CHUNK_ID),
      "Unexpected first-gen chunk in particles file, unpacking not implemented"
    );

    Ok(ParticlesFile {
      version: ParticlesVersionChunk::read::<T>(
        chunks
          .iter()
          .find(|it| it.index == ParticlesVersionChunk::CHUNK_ID)
          .unwrap()
          .clone(),
      )?,
      effects: ParticlesEffectsChunk::read::<T>(
        chunks
          .iter()
          .find(|it| it.index == ParticlesEffectsChunk::CHUNK_ID)
          .unwrap()
          .clone(),
      )?,
      groups: ParticlesGroupsChunk::read::<T>(
        chunks
          .iter()
          .find(|it| it.index == ParticlesGroupsChunk::CHUNK_ID)
          .unwrap()
          .clone(),
      )?,
    })
  }
}

use crate::chunk::reader::ChunkReader;
use crate::chunk::utils::find_chunk_by_id;
use crate::particles_file::particles_effects_chunk::ParticlesEffectsChunk;
use crate::particles_file::particles_firstgen_chunk::ParticlesFirstgenChunk;
use crate::particles_file::particles_groups_chunk::ParticlesGroupsChunk;
use crate::particles_file::particles_header_chunk::ParticlesHeaderChunk;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use fileslice::FileSlice;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesFile {
  pub version: ParticlesHeaderChunk,
  pub effects: ParticlesEffectsChunk,
  pub groups: ParticlesGroupsChunk,
}

impl ParticlesFile {
  /// Read particles xr file from provided path.
  pub fn read_from_path<T: ByteOrder>(path: &Path) -> DatabaseResult<ParticlesFile> {
    Self::read_from_file::<T>(File::open(path)?)
  }

  /// Read particles xr from file.
  pub fn read_from_file<T: ByteOrder>(file: File) -> DatabaseResult<ParticlesFile> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);
    let chunk_ids: Vec<u32> = chunks.iter().map(|it| it.id).collect();

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
      version: ParticlesHeaderChunk::read::<T>(
        &mut find_chunk_by_id(&chunks, ParticlesHeaderChunk::CHUNK_ID)
          .expect("Particle version chunk not found"),
      )?,
      effects: ParticlesEffectsChunk::read::<T>(
        &mut find_chunk_by_id(&chunks, ParticlesEffectsChunk::CHUNK_ID)
          .expect("Particle effects chunk not found"),
      )?,
      groups: ParticlesGroupsChunk::read::<T>(
        &mut find_chunk_by_id(&chunks, ParticlesGroupsChunk::CHUNK_ID)
          .expect("Particle groups chunk not found"),
      )?,
    })
  }

  /// Export unpacked particles file into provided path.
  pub fn export_to_path<T: ByteOrder>(&self, path: &Path) -> DatabaseResult<()> {
    fs::create_dir_all(path)?;

    // todo: Implement.

    Ok(())
  }
}

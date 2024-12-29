use crate::chunk::reader::ChunkReader;
use crate::data::particle::particle_group::ParticleGroup;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesGroupsChunk {
  pub groups: Vec<ParticleGroup>,
}

impl ParticlesGroupsChunk {
  pub const CHUNK_ID: u32 = 4;

  /// Read effects chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> io::Result<ParticlesGroupsChunk> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);
    let mut groups: Vec<ParticleGroup> = Vec::new();

    log::info!(
      "Parsed groups chunk, {:?} bytes, {:?} chunks",
      reader.read_bytes_len(),
      chunks.len()
    );

    for chunk in chunks {
      groups.push(ParticleGroup::read::<T>(chunk)?);
    }

    assert!(reader.is_ended(), "Expect groups chunk to be ended");

    Ok(ParticlesGroupsChunk { groups })
  }
}

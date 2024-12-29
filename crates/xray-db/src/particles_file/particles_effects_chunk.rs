use crate::chunk::reader::ChunkReader;
use crate::data::particle::particle_effect::ParticleEffect;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesEffectsChunk {
  pub particles: Vec<ParticleEffect>,
}

impl ParticlesEffectsChunk {
  pub const CHUNK_ID: u32 = 3;

  /// Read effects chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> io::Result<ParticlesEffectsChunk> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);
    let mut particles: Vec<ParticleEffect> = Vec::new();

    log::info!(
      "Parsing effects chunk, {:?} bytes, {:?} chunks",
      reader.read_bytes_len(),
      chunks.len()
    );

    for chunk in chunks {
      particles.push(ParticleEffect::read::<T>(chunk)?);
    }

    assert!(reader.is_ended(), "Expect effects chunk to be ended");

    Ok(ParticlesEffectsChunk { particles })
  }
}

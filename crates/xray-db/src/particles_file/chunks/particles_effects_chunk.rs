use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_effect::ParticleEffect;
use crate::export::file::create_export_file;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesEffectsChunk {
  pub effects: Vec<ParticleEffect>,
}

impl ParticlesEffectsChunk {
  pub const CHUNK_ID: u32 = 3;

  /// Read effects chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(reader);
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

    Ok(Self { effects: particles })
  }

  /// Write particle effects data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    for effect in &self.effects {
      let mut effect_writer: ChunkWriter = ChunkWriter::new();

      effect.write::<T>(&mut effect_writer)?;

      writer.write_all(effect_writer.flush_chunk_into_buffer::<T>(0)?.as_slice())?;
    }

    Ok(())
  }

  /// Import particles effects data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<Self> {
    todo!("Implement");
  }

  /// Export particles effects data into provided path.
  pub fn export(&self, path: &Path) -> DatabaseResult<()> {
    let mut particles_effects_config: Ltx = Ltx::new();

    for effect in &self.effects {
      effect.export(&effect.name, &mut particles_effects_config)?;
    }

    particles_effects_config.write_to(&mut create_export_file(&path.join("effects.ltx"))?)?;

    Ok(())
  }
}

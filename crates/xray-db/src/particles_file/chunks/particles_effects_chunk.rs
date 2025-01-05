use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::META_TYPE_FIELD;
use crate::data::particle::particle_effect::ParticleEffect;
use crate::export::file::{create_export_file, open_ltx_config};
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
    let mut effects: Vec<ParticleEffect> = Vec::new();

    log::info!(
      "Parsing effects chunk, {:?} bytes, {:?} chunks",
      reader.read_bytes_len(),
      chunks.len()
    );

    for mut chunk in chunks {
      effects.push(ParticleEffect::read::<T>(&mut chunk)?);
    }

    effects.sort_by(|first, second| first.name.cmp(&second.name));

    assert!(reader.is_ended(), "Expect effects chunk to be ended");

    Ok(Self { effects })
  }

  /// Write particle effects data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    for (index, effect) in self.effects.iter().enumerate() {
      let mut effect_writer: ChunkWriter = ChunkWriter::new();

      effect.write::<T>(&mut effect_writer)?;

      writer.write_all(
        effect_writer
          .flush_chunk_into_buffer::<T>(index)?
          .as_slice(),
      )?;
    }

    log::info!(
      "Written effects chunk, {} bytes, {} chunks",
      writer.bytes_written(),
      self.effects.len()
    );

    Ok(())
  }

  /// Import particles effects data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<Self> {
    log::info!("Importing particles effects: {:?}", path);

    let ltx: Ltx = open_ltx_config(&path.join("effects.ltx"))?;
    let mut effects: Vec<ParticleEffect> = Vec::new();

    for (section_name, section) in &ltx {
      if let Some(meta_field) = section.get(META_TYPE_FIELD) {
        if meta_field == ParticleEffect::META_TYPE {
          effects.push(ParticleEffect::import(section_name, &ltx)?);
        }
      }
    }

    effects.sort_by(|first, second| first.name.cmp(&second.name));

    Ok(Self { effects })
  }

  /// Export particles effects data into provided path.
  pub fn export(&self, path: &Path) -> DatabaseResult {
    let mut particles_effects_ltx: Ltx = Ltx::new();

    for effect in &self.effects {
      effect.export(&effect.name, &mut particles_effects_ltx)?;
    }

    particles_effects_ltx.write_to(&mut create_export_file(&path.join("effects.ltx"))?)?;

    log::info!("Exported effects chunk");

    Ok(())
  }
}

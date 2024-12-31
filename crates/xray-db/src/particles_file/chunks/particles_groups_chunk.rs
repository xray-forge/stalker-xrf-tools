use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_group::ParticleGroup;
use crate::export::file::create_export_file;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesGroupsChunk {
  pub groups: Vec<ParticleGroup>,
}

impl ParticlesGroupsChunk {
  pub const CHUNK_ID: u32 = 4;

  /// Read effects chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(reader);
    let mut groups: Vec<ParticleGroup> = Vec::new();

    log::info!(
      "Parsed groups chunk, {:?} bytes, {:?} chunks",
      reader.read_bytes_len(),
      chunks.len()
    );

    for mut chunk in chunks {
      groups.push(ParticleGroup::read::<T>(&mut chunk)?);
    }

    assert!(reader.is_ended(), "Expect groups chunk to be ended");

    Ok(Self { groups })
  }

  /// Write particle groups data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    for group in &self.groups {
      let mut group_writer: ChunkWriter = ChunkWriter::new();

      group.write::<T>(&mut group_writer)?;

      writer.write_all(group_writer.flush_chunk_into_buffer::<T>(0)?.as_slice())?;
    }

    Ok(())
  }

  /// Import particles groups data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<Self> {
    todo!("Implement");
  }

  /// Export particles groups data into provided path.
  pub fn export(&self, path: &Path) -> DatabaseResult<()> {
    let mut particles_effects_config: Ltx = Ltx::new();

    for group in &self.groups {
      group.export(&group.name, &mut particles_effects_config)?;
    }

    particles_effects_config.write_to(&mut create_export_file(&path.join("groups.ltx"))?)?;

    Ok(())
  }
}

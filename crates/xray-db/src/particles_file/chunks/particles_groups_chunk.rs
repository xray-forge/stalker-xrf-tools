use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::META_TYPE_FIELD;
use crate::data::particle::particle_group::ParticleGroup;
use crate::export::file::{create_export_file, open_ltx_config};
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

    groups.sort_by(|first, second| first.name.cmp(&second.name));

    assert!(reader.is_ended(), "Expect groups chunk to be ended");

    Ok(Self { groups })
  }

  /// Write particle groups data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    for (index, group) in self.groups.iter().enumerate() {
      let mut group_writer: ChunkWriter = ChunkWriter::new();

      group.write::<T>(&mut group_writer)?;

      writer.write_all(group_writer.flush_chunk_into_buffer::<T>(index)?.as_slice())?;
    }

    log::info!("Written groups chunk, {} bytes", writer.bytes_written());

    Ok(())
  }

  /// Import particles groups data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<Self> {
    log::info!("Importing particles groups: {:?}", path);

    let ltx: Ltx = open_ltx_config(&path.join("groups.ltx"))?;
    let mut groups: Vec<ParticleGroup> = Vec::new();

    for (section_name, section) in &ltx {
      if let Some(meta_field) = section.get(META_TYPE_FIELD) {
        if meta_field == ParticleGroup::META_TYPE {
          groups.push(ParticleGroup::import(section_name, &ltx)?);
        }
      }
    }

    groups.sort_by(|first, second| first.name.cmp(&second.name));

    Ok(Self { groups })
  }

  /// Export particles groups data into provided path.
  pub fn export(&self, path: &Path) -> DatabaseResult {
    let mut particles_effects_ltx: Ltx = Ltx::new();

    for group in &self.groups {
      group.export(&group.name, &mut particles_effects_ltx)?;
    }

    particles_effects_ltx.write_to(&mut create_export_file(&path.join("groups.ltx"))?)?;

    log::info!("Exported groups chunk");

    Ok(())
  }
}
